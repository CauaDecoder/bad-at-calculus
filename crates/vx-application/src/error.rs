use vx_domain::DomainError;

use crate::ports::ParseError;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ApplicationError {
    #[error("invalid expression: {0}")]
    InvalidExpression(#[from] ParseError),

    #[error(transparent)]
    Math(#[from] DomainError),

    /// A single input field (vector component, matrix entry, bound…) is invalid.
    /// `field` is a stable id such as `u.x`, `a[1][2]` or `lower`.
    #[error("invalid field `{field}`: {cause}")]
    InvalidField {
        field: String,
        cause: Box<ApplicationError>,
    },

    #[error("{0}")]
    InvalidRequest(String),
}

impl ApplicationError {
    pub fn in_field(self, field: impl Into<String>) -> Self {
        Self::InvalidField {
            field: field.into(),
            cause: Box::new(self),
        }
    }
}
