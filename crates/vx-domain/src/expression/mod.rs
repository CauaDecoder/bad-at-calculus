//! Abstract syntax tree (AST) for mathematical expressions.
//!
//! The parser (in `vx-infra`) turns text like `x^2 - 3*x` into an [`Expr`];
//! everything else in the app works with the tree, never with raw strings.

mod latex;
mod math_function;
mod piecewise;
mod simplify;

pub use math_function::MathFunction;
pub use piecewise::{Comparison, Condition, PiecewiseBranch, Relation};

use crate::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Constant(Constant),
    Variable(String),
    Negate(Box<Expr>),
    Binary {
        operator: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        function: MathFunction,
        argument: Box<Expr>,
    },
    /// `{x < 1: x^2; x >= 1: 2x}` — the first branch whose condition holds wins.
    Piecewise(Vec<PiecewiseBranch>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
    Pi,
    E,
}

impl Constant {
    pub fn value(self) -> f64 {
        match self {
            Self::Pi => std::f64::consts::PI,
            Self::E => std::f64::consts::E,
        }
    }
}

impl Expr {
    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    pub fn negate(operand: Expr) -> Self {
        Self::Negate(Box::new(operand))
    }

    pub fn binary(operator: BinaryOperator, left: Expr, right: Expr) -> Self {
        Self::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn sum(left: Expr, right: Expr) -> Self {
        Self::binary(BinaryOperator::Add, left, right)
    }

    pub fn subtract(left: Expr, right: Expr) -> Self {
        Self::binary(BinaryOperator::Subtract, left, right)
    }

    pub fn multiply(left: Expr, right: Expr) -> Self {
        Self::binary(BinaryOperator::Multiply, left, right)
    }

    pub fn divide(left: Expr, right: Expr) -> Self {
        Self::binary(BinaryOperator::Divide, left, right)
    }

    pub fn power(base: Expr, exponent: Expr) -> Self {
        Self::binary(BinaryOperator::Power, base, exponent)
    }

    pub fn call(function: MathFunction, argument: Expr) -> Self {
        Self::Call {
            function,
            argument: Box::new(argument),
        }
    }

    /// True when the expression changes with `variable` (otherwise it is a constant).
    pub fn depends_on(&self, variable: &str) -> bool {
        match self {
            Self::Number(_) | Self::Constant(_) => false,
            Self::Variable(name) => name == variable,
            Self::Negate(operand) => operand.depends_on(variable),
            Self::Binary { left, right, .. } => {
                left.depends_on(variable) || right.depends_on(variable)
            }
            Self::Call { argument, .. } => argument.depends_on(variable),
            Self::Piecewise(branches) => branches.iter().any(|branch| branch.depends_on(variable)),
        }
    }

    /// Evaluates an expression without variables, e.g. `sqrt(2)/2` or `pi/3`.
    pub fn evaluate_constant(&self) -> Result<f64, DomainError> {
        self.evaluate_at("", 0.0)
    }

    /// Evaluates the expression for a single-variable function `f(variable)`.
    pub fn evaluate_at(&self, variable: &str, value: f64) -> Result<f64, DomainError> {
        self.evaluate_with(&|name| (name == variable).then_some(value))
    }

    /// Evaluates with explicit bindings, including variables in piecewise conditions.
    pub fn evaluate_with(
        &self,
        binding: &impl Fn(&str) -> Option<f64>,
    ) -> Result<f64, DomainError> {
        let result = match self {
            Self::Number(number) => *number,
            Self::Constant(constant) => constant.value(),
            Self::Variable(name) => {
                binding(name).ok_or_else(|| DomainError::UnknownVariable(name.clone()))?
            }
            Self::Negate(operand) => -operand.evaluate_with(binding)?,
            Self::Binary {
                operator,
                left,
                right,
            } => {
                let left_value = left.evaluate_with(binding)?;
                let right_value = right.evaluate_with(binding)?;
                operator.apply(left_value, right_value)?
            }
            Self::Call { function, argument } => function.apply(argument.evaluate_with(binding)?),
            Self::Piecewise(branches) => piecewise::evaluate(branches, binding)?,
        };
        ensure_finite(result)
    }
}

impl BinaryOperator {
    fn apply(self, left: f64, right: f64) -> Result<f64, DomainError> {
        match self {
            Self::Add => Ok(left + right),
            Self::Subtract => Ok(left - right),
            Self::Multiply => Ok(left * right),
            Self::Divide if right == 0.0 => Err(DomainError::DivisionByZero),
            Self::Divide => Ok(left / right),
            Self::Power => Ok(left.powf(right)),
        }
    }
}

fn ensure_finite(value: f64) -> Result<f64, DomainError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(DomainError::UndefinedResult)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x() -> Expr {
        Expr::variable("x")
    }

    fn x_squared() -> Expr {
        Expr::power(x(), Expr::Number(2.0))
    }

    #[test]
    fn evaluates_x_squared() {
        assert_eq!(x_squared().evaluate_at("x", 3.0), Ok(9.0));
    }

    #[test]
    fn evaluates_function_calls() {
        let cosine = Expr::call(MathFunction::Cos, x());
        assert_eq!(cosine.evaluate_at("x", 0.0), Ok(1.0));
    }

    #[test]
    fn evaluates_constants() {
        let pi = Expr::Constant(Constant::Pi);
        assert_eq!(pi.evaluate_constant(), Ok(std::f64::consts::PI));
    }

    #[test]
    fn rejects_unknown_variable() {
        assert_eq!(
            Expr::variable("y").evaluate_at("x", 1.0),
            Err(DomainError::UnknownVariable("y".into()))
        );
    }

    #[test]
    fn rejects_division_by_zero() {
        let expression = Expr::divide(Expr::Number(1.0), x());
        assert_eq!(
            expression.evaluate_at("x", 0.0),
            Err(DomainError::DivisionByZero)
        );
    }

    #[test]
    fn logarithm_of_negative_number_is_undefined() {
        let logarithm = Expr::call(MathFunction::Ln, x());
        assert_eq!(
            logarithm.evaluate_at("x", -1.0),
            Err(DomainError::UndefinedResult)
        );
    }

    #[test]
    fn knows_which_expressions_depend_on_the_variable() {
        assert!(x_squared().depends_on("x"));
        assert!(!Expr::Constant(Constant::Pi).depends_on("x"));
    }
}
