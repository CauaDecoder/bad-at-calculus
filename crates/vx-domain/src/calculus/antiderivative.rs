//! Symbolic antiderivatives for the table of immediate integrals, including
//! linear substitution (`sin(3x + 1)`, `(2x − 5)^4`).
//!
//! Anything outside the table returns `None`; the caller then relies on the
//! numerical integral. Every result is double-checked by differentiating it.

use super::{differentiate, RealFunction};
use crate::{BinaryOperator, Constant, Expr, MathFunction};

const VERIFICATION_POINTS: [f64; 5] = [-2.3, -0.7, 0.4, 1.3, 2.9];
const VERIFICATION_TOLERANCE: f64 = 1e-5;
const MIN_VERIFIED_POINTS: usize = 2;

/// `F` with `F' = f`, without the `+ C`.
pub fn antiderivative(expression: &Expr, variable: &str) -> Option<Expr> {
    let candidate = integrate_symbolically(expression, variable)?.simplified();
    verify(expression, &candidate, variable).then_some(candidate)
}

fn integrate_symbolically(expression: &Expr, variable: &str) -> Option<Expr> {
    let x = || Expr::variable(variable);
    if !expression.depends_on(variable) {
        return Some(Expr::multiply(expression.clone(), x()));
    }
    match expression {
        Expr::Variable(_) => Some(Expr::divide(
            Expr::power(x(), Expr::Number(2.0)),
            Expr::Number(2.0),
        )),
        Expr::Negate(operand) => Some(Expr::negate(integrate_symbolically(operand, variable)?)),
        Expr::Binary {
            operator,
            left,
            right,
        } => integrate_binary(*operator, left, right, variable),
        Expr::Call { function, argument } => integrate_call(*function, argument, variable),
        _ => None,
    }
}

fn integrate_binary(operator: BinaryOperator, u: &Expr, v: &Expr, variable: &str) -> Option<Expr> {
    let integrate = |expression: &Expr| integrate_symbolically(expression, variable);
    let (u_varies, v_varies) = (u.depends_on(variable), v.depends_on(variable));
    match operator {
        BinaryOperator::Add => Some(Expr::sum(integrate(u)?, integrate(v)?)),
        BinaryOperator::Subtract => Some(Expr::subtract(integrate(u)?, integrate(v)?)),
        BinaryOperator::Multiply if !u_varies => Some(Expr::multiply(u.clone(), integrate(v)?)),
        BinaryOperator::Multiply if !v_varies => Some(Expr::multiply(v.clone(), integrate(u)?)),
        BinaryOperator::Divide if !v_varies => Some(Expr::divide(integrate(u)?, v.clone())),
        BinaryOperator::Divide if !u_varies => {
            // c / g(x) = c · g(x)^(−1)
            let reciprocal = Expr::power(v.clone(), Expr::Number(-1.0));
            Some(Expr::multiply(u.clone(), integrate(&reciprocal)?))
        }
        BinaryOperator::Power if !v_varies => integrate_power(u, v, variable),
        BinaryOperator::Power if !u_varies => integrate_exponential(u, v, variable),
        _ => None,
    }
}

/// ∫ (ax + b)^n dx = (ax + b)^(n+1) / (a(n+1)), or ln|ax + b| / a when n = −1.
fn integrate_power(base: &Expr, exponent: &Expr, variable: &str) -> Option<Expr> {
    let slope = linear_coefficient(base, variable)?;
    let n = exponent.evaluate_constant().ok()?;
    if n == -1.0 {
        let logarithm = Expr::call(
            MathFunction::Ln,
            Expr::call(MathFunction::Abs, base.clone()),
        );
        return Some(Expr::divide(logarithm, Expr::Number(slope)));
    }
    let raised = Expr::sum(exponent.clone(), Expr::Number(1.0));
    Some(Expr::divide(
        Expr::power(base.clone(), raised.clone()),
        Expr::multiply(Expr::Number(slope), raised),
    ))
}

/// ∫ a^(kx + b) dx = a^(kx + b) / (k ln a)  (and e^u / k).
fn integrate_exponential(base: &Expr, exponent: &Expr, variable: &str) -> Option<Expr> {
    let slope = linear_coefficient(exponent, variable)?;
    let whole = Expr::power(base.clone(), exponent.clone());
    let divisor = if *base == Expr::Constant(Constant::E) {
        Expr::Number(slope)
    } else {
        Expr::multiply(
            Expr::Number(slope),
            Expr::call(MathFunction::Ln, base.clone()),
        )
    };
    Some(Expr::divide(whole, divisor))
}

fn integrate_call(function: MathFunction, u: &Expr, variable: &str) -> Option<Expr> {
    let slope = linear_coefficient(u, variable)?;
    let call = |function: MathFunction| Expr::call(function, u.clone());
    let primitive = match function {
        MathFunction::Sin => Expr::negate(call(MathFunction::Cos)),
        MathFunction::Cos => call(MathFunction::Sin),
        MathFunction::Exp => call(MathFunction::Exp),
        MathFunction::Tan => Expr::negate(Expr::call(
            MathFunction::Ln,
            Expr::call(MathFunction::Abs, call(MathFunction::Cos)),
        )),
        MathFunction::Sqrt => Expr::multiply(
            Expr::divide(Expr::Number(2.0), Expr::Number(3.0)),
            Expr::power(
                u.clone(),
                Expr::divide(Expr::Number(3.0), Expr::Number(2.0)),
            ),
        ),
        MathFunction::Cbrt => Expr::multiply(
            Expr::divide(Expr::Number(3.0), Expr::Number(4.0)),
            Expr::multiply(u.clone(), call(MathFunction::Cbrt)),
        ),
        MathFunction::Ln => {
            Expr::subtract(Expr::multiply(u.clone(), call(MathFunction::Ln)), u.clone())
        }
        MathFunction::Log10 => Expr::divide(
            Expr::subtract(Expr::multiply(u.clone(), call(MathFunction::Ln)), u.clone()),
            Expr::call(MathFunction::Ln, Expr::Number(10.0)),
        ),
        MathFunction::Abs => Expr::divide(
            Expr::multiply(u.clone(), call(MathFunction::Abs)),
            Expr::Number(2.0),
        ),
    };
    Some(Expr::divide(primitive, Expr::Number(slope)))
}

/// `a` when `u = ax + b` (a ≠ 0), i.e. when `u'` is a non-zero constant.
fn linear_coefficient(u: &Expr, variable: &str) -> Option<f64> {
    let derivative = differentiate(u, variable).derivative;
    if derivative.depends_on(variable) {
        return None;
    }
    let slope = derivative.evaluate_constant().ok()?;
    (slope != 0.0).then_some(slope)
}

/// `F'(x) ≈ f(x)` wherever both are defined.
fn verify(expression: &Expr, candidate: &Expr, variable: &str) -> bool {
    let f = RealFunction::new(variable, expression.clone());
    let primitive = RealFunction::new(variable, candidate.clone());
    let mut verified = 0;
    for x in VERIFICATION_POINTS {
        let (Ok(expected), Ok(actual)) = (f.value_at(x), primitive.derivative_at(x)) else {
            continue;
        };
        if (expected - actual).abs() > VERIFICATION_TOLERANCE * (1.0 + expected.abs()) {
            return false;
        }
        verified += 1;
    }
    verified >= MIN_VERIFIED_POINTS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x() -> Expr {
        Expr::variable("x")
    }

    fn number(value: f64) -> Expr {
        Expr::Number(value)
    }

    fn primitive_latex(expression: &Expr) -> Option<String> {
        antiderivative(expression, "x").map(|primitive| primitive.to_latex())
    }

    #[test]
    fn power_rule_for_integrals() {
        assert_eq!(
            primitive_latex(&Expr::power(x(), number(2.0))),
            Some(r"\frac{x^{3}}{3}".to_owned())
        );
    }

    #[test]
    fn polynomial_by_linearity() {
        let polynomial = Expr::sum(
            Expr::multiply(number(3.0), Expr::power(x(), number(2.0))),
            Expr::multiply(number(2.0), x()),
        );
        assert!(antiderivative(&polynomial, "x").is_some());
    }

    #[test]
    fn one_over_x_is_log_of_absolute_value() {
        let latex = primitive_latex(&Expr::divide(number(1.0), x()));
        assert_eq!(latex, Some(r"\ln\left(\left|x\right|\right)".to_owned()));
    }

    #[test]
    fn linear_substitution_in_sine() {
        let expression = Expr::call(
            MathFunction::Sin,
            Expr::sum(Expr::multiply(number(3.0), x()), number(1.0)),
        );
        assert!(antiderivative(&expression, "x").is_some());
    }

    #[test]
    fn products_of_non_constant_factors_are_not_in_the_table() {
        let expression = Expr::multiply(x(), Expr::call(MathFunction::Sin, x()));
        assert_eq!(antiderivative(&expression, "x"), None);
    }
}
