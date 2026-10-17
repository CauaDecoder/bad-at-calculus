//! Hand-written fakes of the ports, shared by use case tests.

use vx_domain::{BinaryOperator, Expr};

use crate::ports::{ExpressionParser, ParseError};

/// Always returns `x^2`, whatever the input.
pub struct FakeParserReturningXSquared;

impl ExpressionParser for FakeParserReturningXSquared {
    fn parse(&self, source: &str) -> Result<Expr, ParseError> {
        if let Ok(number) = source.trim().parse::<f64>() {
            return Ok(Expr::Number(number));
        }
        Ok(Expr::binary(
            BinaryOperator::Power,
            Expr::variable("x"),
            Expr::Number(2.0),
        ))
    }
}

/// Understands plain decimal numbers only; anything else is a variable `x`.
pub struct NumberParser;

impl ExpressionParser for NumberParser {
    fn parse(&self, source: &str) -> Result<Expr, ParseError> {
        Ok(source
            .trim()
            .parse::<f64>()
            .map_or_else(|_| Expr::variable("x"), Expr::Number))
    }
}

pub struct FailingParser;

impl ExpressionParser for FailingParser {
    fn parse(&self, _source: &str) -> Result<Expr, ParseError> {
        Err(ParseError::new("unexpected token", 0))
    }
}
