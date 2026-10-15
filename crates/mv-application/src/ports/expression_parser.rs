use mv_domain::Expr;

/// Turns user-typed text (`"x^2 - 3x"`) into a domain [`Expr`].
pub trait ExpressionParser {
    fn parse(&self, source: &str) -> Result<Expr, ParseError>;
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("{message} (at position {position})")]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }
}
