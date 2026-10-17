//! Calculus: functions, limits and continuity, derivatives and integrals.

mod antiderivative;
mod critical_points;
mod differentiation;
mod integration;
mod interval;
mod limits;
mod sampling;

pub use antiderivative::antiderivative;
pub use critical_points::{CriticalPoint, CriticalPointKind};
pub use differentiation::{differentiate, Derivation, DerivationStep, DerivativeRule};
pub use integration::{DefiniteIntegral, RiemannMethod, RiemannPiece, RiemannSum};
pub use interval::Interval;
pub use limits::{
    ApproachSample, ContinuityAnalysis, DiscontinuityKind, LimitAnalysis, LimitTarget, LimitValue,
    OneSidedLimit,
};
pub use sampling::FunctionSamples;

use crate::{DomainError, Expr, Point2};

/// Step used by the central difference. Small enough for precision,
/// large enough to avoid floating-point cancellation.
const DERIVATIVE_STEP: f64 = 1e-5;

/// A real function of one variable, `f(variable) = expression`.
#[derive(Debug, Clone, PartialEq)]
pub struct RealFunction {
    variable: String,
    expression: Expr,
}

impl RealFunction {
    pub fn new(variable: impl Into<String>, expression: Expr) -> Self {
        Self {
            variable: variable.into(),
            expression,
        }
    }

    pub fn expression(&self) -> &Expr {
        &self.expression
    }

    pub fn variable(&self) -> &str {
        &self.variable
    }

    pub fn value_at(&self, x: f64) -> Result<f64, DomainError> {
        self.expression.evaluate_at(&self.variable, x)
    }

    /// Numerical derivative `f'(x) ≈ [f(x+h) − f(x−h)] / 2h`, used for plots
    /// and critical points. The symbolic one is [`differentiate`].
    pub fn derivative_at(&self, x: f64) -> Result<f64, DomainError> {
        let forward = self.value_at(x + DERIVATIVE_STEP)?;
        let backward = self.value_at(x - DERIVATIVE_STEP)?;
        Ok((forward - backward) / (2.0 * DERIVATIVE_STEP))
    }

    pub fn tangent_line_at(&self, x: f64) -> Result<TangentLine, DomainError> {
        let point = Point2::new(x, self.value_at(x)?);
        let slope = self.derivative_at(x)?;
        Ok(TangentLine { point, slope })
    }
}

/// The line `y = slope · (x − x₀) + y₀` that touches a curve at `point`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TangentLine {
    pub point: Point2,
    pub slope: f64,
}

impl TangentLine {
    pub fn value_at(&self, x: f64) -> f64 {
        self.slope * (x - self.point.x) + self.point.y
    }

    pub fn y_intercept(&self) -> f64 {
        self.value_at(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-6;

    fn x() -> Expr {
        Expr::variable("x")
    }

    fn x_squared() -> RealFunction {
        RealFunction::new("x", Expr::power(x(), Expr::Number(2.0)))
    }

    #[test]
    fn derivative_of_x_squared_is_2x() -> Result<(), DomainError> {
        let slope = x_squared().derivative_at(3.0)?;
        assert!((slope - 6.0).abs() < TOLERANCE);
        Ok(())
    }

    #[test]
    fn tangent_to_x_squared_at_1_crosses_y_axis_at_minus_1() -> Result<(), DomainError> {
        let tangent = x_squared().tangent_line_at(1.0)?;
        assert!((tangent.y_intercept() + 1.0).abs() < TOLERANCE);
        Ok(())
    }
}
