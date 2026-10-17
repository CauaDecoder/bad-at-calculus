use super::{ExpressionForm, Verdict};
use crate::{BinaryOperator as Op, Expr};

/// Checks the submitted AST, not its simplified equivalent. Unknown exponent signs
/// remain indeterminate rather than silently satisfying a structural requirement.
pub fn check_form(expression: &Expr, form: ExpressionForm) -> Verdict {
    let result = match form {
        ExpressionForm::Factored => Some(factored(expression)),
        ExpressionForm::Expanded => expanded(expression),
        ExpressionForm::NoNegativeExponents => nonnegative_exponents(expression),
    };
    match result {
        Some(true) => Verdict::Correct,
        Some(false) => Verdict::Incorrect,
        None => Verdict::Indeterminate,
    }
}

fn factored(expression: &Expr) -> bool {
    match expression {
        Expr::Negate(inner) => factored(inner),
        Expr::Binary {
            operator: Op::Divide,
            left,
            right,
        } => factored(left) && factored(right),
        Expr::Binary {
            operator: Op::Add | Op::Subtract,
            ..
        }
        | Expr::Piecewise(_) => false,
        _ => true,
    }
}

fn expanded(expression: &Expr) -> Option<bool> {
    match expression {
        Expr::Binary {
            operator: Op::Multiply,
            left,
            right,
        } if additive(left) || additive(right) => Some(false),
        Expr::Binary {
            operator: Op::Power,
            left,
            right,
        } if additive(left) => {
            let power = right.evaluate_constant().ok()?;
            if power > 1.0 && power.fract() == 0.0 {
                Some(false)
            } else {
                children(expression, expanded)
            }
        }
        _ => children(expression, expanded),
    }
}

fn additive(expression: &Expr) -> bool {
    match expression {
        Expr::Negate(inner) => additive(inner),
        Expr::Binary {
            operator: Op::Add | Op::Subtract,
            ..
        } => true,
        Expr::Binary {
            operator: Op::Power,
            left,
            right,
        } if right.evaluate_constant() == Ok(1.0) => additive(left),
        _ => false,
    }
}

fn nonnegative_exponents(expression: &Expr) -> Option<bool> {
    let sign = match expression {
        Expr::Binary {
            operator: Op::Power,
            right,
            ..
        } => right.evaluate_constant().ok().map(|value| value >= 0.0),
        _ => Some(true),
    };
    conjunction(sign, children(expression, nonnegative_exponents))
}

fn conjunction(left: Option<bool>, right: Option<bool>) -> Option<bool> {
    match (left, right) {
        (Some(false), _) | (_, Some(false)) => Some(false),
        (Some(true), Some(true)) => Some(true),
        _ => None,
    }
}

fn children(expression: &Expr, check: fn(&Expr) -> Option<bool>) -> Option<bool> {
    match expression {
        Expr::Negate(inner)
        | Expr::Call {
            argument: inner, ..
        } => check(inner),
        Expr::Binary { left, right, .. } => conjunction(check(left), check(right)),
        Expr::Piecewise(branches) => check_branches(branches, check),
        _ => Some(true),
    }
}

fn check_branches(
    branches: &[crate::PiecewiseBranch],
    check: fn(&Expr) -> Option<bool>,
) -> Option<bool> {
    let mut result = Some(true);
    for branch in branches {
        if let Some(condition) = &branch.condition {
            for comparison in &condition.comparisons {
                result = conjunction(
                    result,
                    conjunction(check(&comparison.left), check(&comparison.right)),
                );
            }
        }
        // Unlike try_fold, keep looking after an unknown: a later violation is decisive.
        result = conjunction(result, check(&branch.value));
    }
    result
}
