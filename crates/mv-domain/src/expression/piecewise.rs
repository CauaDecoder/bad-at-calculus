//! Piecewise-defined functions, e.g. `f(x) = x² if x < 1, 2x if x ≥ 1`.

use super::Expr;
use crate::DomainError;

/// One branch: `value` applies where `condition` holds (`None` = otherwise).
#[derive(Debug, Clone, PartialEq)]
pub struct PiecewiseBranch {
    pub condition: Option<Condition>,
    pub value: Expr,
}

/// All comparisons must hold: `-1 < x <= 2` is `[-1 < x, x <= 2]`.
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub comparisons: Vec<Comparison>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub left: Expr,
    pub relation: Relation,
    pub right: Expr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
}

impl Relation {
    fn holds(self, left: f64, right: f64) -> bool {
        match self {
            Self::Less => left < right,
            Self::LessOrEqual => left <= right,
            Self::Greater => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::Equal => left == right,
        }
    }
}

impl PiecewiseBranch {
    pub fn depends_on(&self, variable: &str) -> bool {
        let condition_depends = self.condition.as_ref().is_some_and(|condition| {
            condition.comparisons.iter().any(|comparison| {
                comparison.left.depends_on(variable) || comparison.right.depends_on(variable)
            })
        });
        condition_depends || self.value.depends_on(variable)
    }

    fn applies_at(&self, variable: &str, value: f64) -> Result<bool, DomainError> {
        let Some(condition) = &self.condition else {
            return Ok(true);
        };
        for comparison in &condition.comparisons {
            let left = comparison.left.evaluate_at(variable, value)?;
            let right = comparison.right.evaluate_at(variable, value)?;
            if !comparison.relation.holds(left, right) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

pub(super) fn evaluate(
    branches: &[PiecewiseBranch],
    variable: &str,
    value: f64,
) -> Result<f64, DomainError> {
    for branch in branches {
        if branch.applies_at(variable, value)? {
            return branch.value.evaluate_at(variable, value);
        }
    }
    Err(DomainError::UndefinedResult)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x() -> Expr {
        Expr::variable("x")
    }

    /// `{x < 1: x^2; x >= 1: 2x}`
    fn broken_function() -> Expr {
        Expr::Piecewise(vec![
            PiecewiseBranch {
                condition: Some(Condition {
                    comparisons: vec![Comparison {
                        left: x(),
                        relation: Relation::Less,
                        right: Expr::Number(1.0),
                    }],
                }),
                value: Expr::power(x(), Expr::Number(2.0)),
            },
            PiecewiseBranch {
                condition: Some(Condition {
                    comparisons: vec![Comparison {
                        left: x(),
                        relation: Relation::GreaterOrEqual,
                        right: Expr::Number(1.0),
                    }],
                }),
                value: Expr::multiply(Expr::Number(2.0), x()),
            },
        ])
    }

    #[test]
    fn uses_the_first_branch_whose_condition_holds() {
        assert_eq!(broken_function().evaluate_at("x", 0.5), Ok(0.25));
        assert_eq!(broken_function().evaluate_at("x", 1.0), Ok(2.0));
    }

    #[test]
    fn is_undefined_where_no_branch_applies() {
        let only_positive = Expr::Piecewise(vec![PiecewiseBranch {
            condition: Some(Condition {
                comparisons: vec![Comparison {
                    left: x(),
                    relation: Relation::Greater,
                    right: Expr::Number(0.0),
                }],
            }),
            value: x(),
        }]);
        assert_eq!(
            only_positive.evaluate_at("x", -1.0),
            Err(DomainError::UndefinedResult)
        );
    }
}
