//! Algebraic clean-up so results read like a student would write them:
//! `2·x^(2−1)·1` → `2x`, `0 + x` → `x`, `x·3` → `3x`, `6/4` → `3/2`.
//!
//! It is intentionally a small rule set, not a full computer-algebra system.

use super::{BinaryOperator, Expr, MathFunction, PiecewiseBranch};

const MAX_PASSES: usize = 12;
/// Integer powers above this are left unexpanded (`2^100` stays symbolic).
const MAX_FOLDED_EXPONENT: f64 = 64.0;
const INTEGER_TOLERANCE: f64 = 1e-12;

impl Expr {
    pub fn simplified(&self) -> Expr {
        let mut current = self.clone();
        for _ in 0..MAX_PASSES {
            let next = simplify_once(&current);
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
}

fn simplify_once(expression: &Expr) -> Expr {
    match expression {
        Expr::Number(_) | Expr::Constant(_) | Expr::Variable(_) => expression.clone(),
        Expr::Negate(operand) => simplify_negation(simplify_once(operand)),
        Expr::Binary {
            operator,
            left,
            right,
        } => simplify_binary(*operator, simplify_once(left), simplify_once(right)),
        Expr::Call { function, argument } => simplify_call(*function, simplify_once(argument)),
        Expr::Piecewise(branches) => Expr::Piecewise(
            branches
                .iter()
                .map(|branch| PiecewiseBranch {
                    condition: branch.condition.clone(),
                    value: simplify_once(&branch.value),
                })
                .collect(),
        ),
    }
}

fn simplify_negation(operand: Expr) -> Expr {
    match operand {
        Expr::Number(value) => Expr::Number(-value),
        Expr::Negate(inner) => *inner,
        Expr::Binary {
            operator: BinaryOperator::Subtract,
            left,
            right,
        } => Expr::subtract(*right, *left),
        other => Expr::negate(other),
    }
}

fn simplify_binary(operator: BinaryOperator, left: Expr, right: Expr) -> Expr {
    if let (Expr::Number(a), Expr::Number(b)) = (&left, &right) {
        if let Some(folded) = fold_numbers(operator, *a, *b) {
            return folded;
        }
    }
    match operator {
        BinaryOperator::Add => simplify_sum(left, right),
        BinaryOperator::Subtract => simplify_difference(left, right),
        BinaryOperator::Multiply => simplify_product(left, right),
        BinaryOperator::Divide => simplify_quotient(left, right),
        BinaryOperator::Power => simplify_power(left, right),
    }
}

fn simplify_sum(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (left, right) if is_number(&left, 0.0) => right,
        (left, right) if is_number(&right, 0.0) => left,
        (left, Expr::Negate(right)) => Expr::subtract(left, *right),
        (left, Expr::Number(value)) if value < 0.0 => Expr::subtract(left, Expr::Number(-value)),
        (Expr::Negate(left), right) => Expr::subtract(right, *left),
        (left, right) if left == right => Expr::multiply(Expr::Number(2.0), left),
        (left, right) => Expr::sum(left, right),
    }
}

fn simplify_difference(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (left, right) if is_number(&right, 0.0) => left,
        (left, right) if is_number(&left, 0.0) => Expr::negate(right),
        (left, Expr::Negate(right)) => Expr::sum(left, *right),
        (left, Expr::Number(value)) if value < 0.0 => Expr::sum(left, Expr::Number(-value)),
        (left, right) if left == right => Expr::Number(0.0),
        (left, right) => Expr::subtract(left, right),
    }
}

fn simplify_product(left: Expr, right: Expr) -> Expr {
    match (left, right) {
        (left, right) if is_number(&left, 0.0) || is_number(&right, 0.0) => Expr::Number(0.0),
        (left, right) if is_number(&left, 1.0) => right,
        (left, right) if is_number(&right, 1.0) => left,
        (left, right) if is_number(&left, -1.0) => Expr::negate(right),
        (Expr::Negate(left), right) => Expr::negate(Expr::multiply(*left, right)),
        (left, Expr::Negate(right)) => Expr::negate(Expr::multiply(left, *right)),
        (left, Expr::Number(value)) => Expr::multiply(Expr::Number(value), left),
        (
            Expr::Number(a),
            Expr::Binary {
                operator: BinaryOperator::Multiply,
                left,
                right,
            },
        ) if matches!(*left, Expr::Number(_)) => {
            let b = number_value(&left).unwrap_or(1.0);
            Expr::multiply(Expr::Number(a * b), *right)
        }
        (left, right) if !is_any_number(&left) && leading_coefficient(&right).is_some() => {
            // a · (c · b) → c · (a · b): coefficients travel to the front.
            let (coefficient, rest) = split_coefficient(right);
            Expr::multiply(coefficient, Expr::multiply(left, rest))
        }
        (left, right) if !is_any_number(&right) && leading_coefficient(&left).is_some() => {
            // (c · a) · b → c · (a · b)
            let (coefficient, rest) = split_coefficient(left);
            Expr::multiply(coefficient, Expr::multiply(rest, right))
        }
        (left, right) if is_function_call(&left) && !is_function_call(&right) => {
            // cos(x) · x → x · cos(x): functions go last, as students write them.
            Expr::multiply(right, left)
        }
        (left, right) if left == right => Expr::power(left, Expr::Number(2.0)),
        (left, right) => merge_powers(left, right),
    }
}

/// `c` in a product `c · rest` whose first factor is a number.
fn leading_coefficient(expression: &Expr) -> Option<f64> {
    match expression {
        Expr::Binary {
            operator: BinaryOperator::Multiply,
            left,
            ..
        } => number_value(left),
        _ => None,
    }
}

fn split_coefficient(product: Expr) -> (Expr, Expr) {
    match product {
        Expr::Binary {
            operator: BinaryOperator::Multiply,
            left,
            right,
        } => (*left, *right),
        other => (Expr::Number(1.0), other),
    }
}

fn is_any_number(expression: &Expr) -> bool {
    number_value(expression).is_some()
}

fn is_function_call(expression: &Expr) -> bool {
    matches!(expression, Expr::Call { .. } | Expr::Piecewise(_))
}

/// `x^2 · x` → `x^3`, `x^a · x^b` → `x^(a+b)`.
fn merge_powers(left: Expr, right: Expr) -> Expr {
    let (left_base, left_exponent) = as_power(&left);
    let (right_base, right_exponent) = as_power(&right);
    if left_base == right_base {
        let exponent = Expr::sum(left_exponent, right_exponent);
        return Expr::power(left_base, exponent);
    }
    Expr::multiply(left, right)
}

fn as_power(expression: &Expr) -> (Expr, Expr) {
    match expression {
        Expr::Binary {
            operator: BinaryOperator::Power,
            left,
            right,
        } => ((**left).clone(), (**right).clone()),
        other => (other.clone(), Expr::Number(1.0)),
    }
}

fn simplify_quotient(numerator: Expr, denominator: Expr) -> Expr {
    match (numerator, denominator) {
        (numerator, denominator) if is_number(&denominator, 1.0) => numerator,
        (numerator, denominator) if is_number(&numerator, 0.0) && !is_number(&denominator, 0.0) => {
            Expr::Number(0.0)
        }
        (numerator, denominator) if numerator == denominator => Expr::Number(1.0),
        (Expr::Negate(numerator), denominator) => {
            Expr::negate(Expr::divide(*numerator, denominator))
        }
        (
            Expr::Binary {
                operator: BinaryOperator::Multiply,
                left,
                right,
            },
            Expr::Number(divisor),
        ) if number_value(&left).is_some_and(|value| is_integer(value / divisor)) => {
            let coefficient = number_value(&left).unwrap_or(divisor) / divisor;
            Expr::multiply(Expr::Number(coefficient), *right)
        }
        (numerator, denominator) => Expr::divide(numerator, denominator),
    }
}

fn simplify_power(base: Expr, exponent: Expr) -> Expr {
    match (base, exponent) {
        (base, exponent) if is_number(&exponent, 1.0) => base,
        (_, exponent) if is_number(&exponent, 0.0) => Expr::Number(1.0),
        (base, _) if is_number(&base, 1.0) => Expr::Number(1.0),
        (
            Expr::Binary {
                operator: BinaryOperator::Power,
                left,
                right,
            },
            Expr::Number(outer),
        ) if number_value(&right).is_some() => {
            let inner = number_value(&right).unwrap_or(1.0);
            Expr::power(*left, Expr::Number(inner * outer))
        }
        (base, exponent) => Expr::power(base, exponent),
    }
}

/// Evaluates functions of numbers only when the result is an integer:
/// `sin(0)` → `0`, `sqrt(9)` → `3`, but `sqrt(2)` stays exact.
fn simplify_call(function: MathFunction, argument: Expr) -> Expr {
    if let Expr::Number(value) = argument {
        let result = function.apply(value);
        if result.is_finite() && is_integer(result) {
            return Expr::Number(result.round());
        }
    }
    Expr::call(function, argument)
}

fn fold_numbers(operator: BinaryOperator, a: f64, b: f64) -> Option<Expr> {
    let folded = match operator {
        BinaryOperator::Add => a + b,
        BinaryOperator::Subtract => a - b,
        BinaryOperator::Multiply => a * b,
        BinaryOperator::Divide => return fold_fraction(a, b),
        BinaryOperator::Power => return fold_power(a, b),
    };
    Some(Expr::Number(folded))
}

/// Keeps `1/3` as a fraction (exact), but reduces it: `6/4` → `3/2`, `8/4` → `2`.
fn fold_fraction(numerator: f64, denominator: f64) -> Option<Expr> {
    if denominator == 0.0 {
        return None;
    }
    let quotient = numerator / denominator;
    if is_integer(quotient) {
        return Some(Expr::Number(quotient.round()));
    }
    if !(is_integer(numerator) && is_integer(denominator)) {
        return Some(Expr::Number(quotient));
    }
    let divisor = greatest_common_divisor(numerator.abs(), denominator.abs());
    let sign = if denominator < 0.0 { -1.0 } else { 1.0 };
    let reduced = Expr::divide(
        Expr::Number(sign * numerator / divisor),
        Expr::Number(sign * denominator / divisor),
    );
    if reduced == Expr::divide(Expr::Number(numerator), Expr::Number(denominator)) {
        return None;
    }
    Some(reduced)
}

fn fold_power(base: f64, exponent: f64) -> Option<Expr> {
    let is_small_integer_exponent = is_integer(exponent) && exponent.abs() <= MAX_FOLDED_EXPONENT;
    if !is_small_integer_exponent {
        return None;
    }
    if exponent >= 0.0 {
        return Some(Expr::Number(base.powf(exponent)));
    }
    fold_fraction(1.0, base.powf(-exponent)).or_else(|| {
        Some(Expr::divide(
            Expr::Number(1.0),
            Expr::Number(base.powf(-exponent)),
        ))
    })
}

fn greatest_common_divisor(a: f64, b: f64) -> f64 {
    let (mut a, mut b) = (a.round() as i128, b.round() as i128);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1) as f64
}

fn is_number(expression: &Expr, expected: f64) -> bool {
    number_value(expression) == Some(expected)
}

fn number_value(expression: &Expr) -> Option<f64> {
    match expression {
        Expr::Number(value) => Some(*value),
        _ => None,
    }
}

fn is_integer(value: f64) -> bool {
    (value - value.round()).abs() < INTEGER_TOLERANCE
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

    #[test]
    fn folds_power_rule_leftovers() {
        let raw = Expr::multiply(
            Expr::multiply(
                number(2.0),
                Expr::power(x(), Expr::subtract(number(2.0), number(1.0))),
            ),
            number(1.0),
        );
        assert_eq!(raw.simplified(), Expr::multiply(number(2.0), x()));
    }

    #[test]
    fn puts_coefficient_first() {
        let raw = Expr::multiply(x(), number(3.0));
        assert_eq!(raw.simplified().to_latex(), "3 x");
    }

    #[test]
    fn removes_neutral_elements() {
        assert_eq!(Expr::sum(number(0.0), x()).simplified(), x());
        assert_eq!(Expr::power(x(), number(1.0)).simplified(), x());
        assert_eq!(Expr::multiply(number(0.0), x()).simplified(), number(0.0));
    }

    #[test]
    fn keeps_fractions_exact_and_reduced() {
        assert_eq!(
            Expr::divide(number(6.0), number(4.0))
                .simplified()
                .to_latex(),
            r"\frac{3}{2}"
        );
        assert_eq!(
            Expr::divide(number(8.0), number(4.0)).simplified(),
            number(2.0)
        );
    }

    #[test]
    fn negative_integer_powers_become_fractions() {
        assert_eq!(
            Expr::power(number(2.0), number(-1.0))
                .simplified()
                .to_latex(),
            r"\frac{1}{2}"
        );
    }

    #[test]
    fn adding_a_negative_becomes_subtraction() {
        let raw = Expr::sum(x(), Expr::negate(number(3.0)));
        assert_eq!(raw.simplified().to_latex(), "x - 3");
    }

    #[test]
    fn merges_powers_of_the_same_base() {
        let raw = Expr::multiply(Expr::power(x(), number(2.0)), x());
        assert_eq!(raw.simplified().to_latex(), "x^{3}");
    }

    #[test]
    fn evaluates_functions_only_when_exact() {
        assert_eq!(
            Expr::call(MathFunction::Cos, number(0.0)).simplified(),
            number(1.0)
        );
        assert_eq!(
            Expr::call(MathFunction::Sqrt, number(2.0))
                .simplified()
                .to_latex(),
            r"\sqrt{2}"
        );
    }
}
