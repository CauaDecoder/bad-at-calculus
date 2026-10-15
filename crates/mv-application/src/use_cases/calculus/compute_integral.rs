use mv_domain::{
    calculus::{
        antiderivative, DefiniteIntegral, Interval, RealFunction, RiemannMethod, RiemannSum,
    },
    Expr,
};

use super::{evaluate_number, parse_function};
use crate::{ports::ExpressionParser, ApplicationError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiemannRequest {
    pub piece_count: usize,
    pub method: RiemannMethod,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegralRequest {
    pub expression: String,
    pub variable: String,
    /// Bounds as typed: `0`, `pi`, `-1/2`.
    pub lower: String,
    pub upper: String,
    pub riemann: Option<RiemannRequest>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegralReport {
    pub lower: f64,
    pub upper: f64,
    pub integral: DefiniteIntegral,
    /// F with F' = f, when f is in the table of immediate integrals.
    pub antiderivative: Option<Expr>,
    /// (F(a), F(b)) for the Fundamental Theorem of Calculus.
    pub antiderivative_at_bounds: Option<(f64, f64)>,
    pub riemann: Option<RiemannSum>,
}

/// "How much is ∫ₐᵇ f(x) dx, and how do Riemann sums approach it?"
pub struct ComputeIntegral<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> ComputeIntegral<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &IntegralRequest) -> Result<IntegralReport, ApplicationError> {
        let function = parse_function(&self.parser, &request.expression, &request.variable)?;
        let lower = evaluate_number(&self.parser, &request.lower, "lower")?;
        let upper = evaluate_number(&self.parser, &request.upper, "upper")?;
        let antiderivative = antiderivative(function.expression(), &request.variable);
        let antiderivative_at_bounds = antiderivative
            .as_ref()
            .and_then(|primitive| evaluate_at_bounds(primitive, &request.variable, lower, upper));

        if lower == upper {
            return Ok(IntegralReport {
                lower,
                upper,
                integral: DefiniteIntegral {
                    value: 0.0,
                    estimated_error: 0.0,
                },
                antiderivative,
                antiderivative_at_bounds,
                riemann: None,
            });
        }

        // ∫ₐᵇ = −∫ᵇₐ: integrate over the ordered interval, then fix the sign.
        let orientation = if lower < upper { 1.0 } else { -1.0 };
        let interval = Interval::new(lower.min(upper), lower.max(upper))?;
        let ordered = function.integrate(interval)?;
        let riemann = request
            .riemann
            .map(|riemann| oriented_riemann(&function, interval, riemann, orientation))
            .transpose()?;

        Ok(IntegralReport {
            lower,
            upper,
            integral: DefiniteIntegral {
                value: orientation * ordered.value,
                estimated_error: ordered.estimated_error,
            },
            antiderivative,
            antiderivative_at_bounds,
            riemann,
        })
    }
}

fn oriented_riemann(
    function: &RealFunction,
    interval: Interval,
    request: RiemannRequest,
    orientation: f64,
) -> Result<RiemannSum, ApplicationError> {
    let mut sum = function.riemann_sum(interval, request.piece_count, request.method)?;
    sum.value *= orientation;
    Ok(sum)
}

fn evaluate_at_bounds(
    primitive: &Expr,
    variable: &str,
    lower: f64,
    upper: f64,
) -> Option<(f64, f64)> {
    let lower_value = primitive.evaluate_at(variable, lower).ok()?;
    let upper_value = primitive.evaluate_at(variable, upper).ok()?;
    Some((lower_value, upper_value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::FakeParserReturningXSquared;

    fn request(lower: &str, upper: &str) -> IntegralRequest {
        IntegralRequest {
            expression: "x^2".into(),
            variable: "x".into(),
            lower: lower.into(),
            upper: upper.into(),
            riemann: Some(RiemannRequest {
                piece_count: 4,
                method: RiemannMethod::Midpoint,
            }),
        }
    }

    #[test]
    fn fundamental_theorem_matches_numeric_value() -> Result<(), ApplicationError> {
        let report =
            ComputeIntegral::new(FakeParserReturningXSquared).execute(&request("0", "3"))?;
        assert!((report.integral.value - 9.0).abs() < 1e-9);
        let (at_lower, at_upper) = report.antiderivative_at_bounds.unwrap_or_default();
        assert!((at_upper - at_lower - 9.0).abs() < 1e-9);
        Ok(())
    }

    #[test]
    fn swapping_bounds_changes_the_sign() -> Result<(), ApplicationError> {
        let report =
            ComputeIntegral::new(FakeParserReturningXSquared).execute(&request("3", "0"))?;
        assert!((report.integral.value + 9.0).abs() < 1e-9);
        assert!(report.riemann.is_some_and(|sum| sum.value < 0.0));
        Ok(())
    }
}
