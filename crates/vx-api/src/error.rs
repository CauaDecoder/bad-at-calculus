use std::fmt::Display;

use serde::Serialize;
use vx_application::ApplicationError;
use vx_domain::DomainError;

/// Structured error sent to the UI, which translates `code` into Portuguese.
///
/// Codes: `syntax`, `bad-request`, `unknown-command`, `invalid-request`, and one
/// per math rule (`division-by-zero`, `undefined-result`, `zero-vector`,
/// `singular-matrix`, `not-integrable`, …). `field` names the input at fault
/// (`u.y`, `a[1][2]`, `lower`); `position` is the character index of a syntax error.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl ApiError {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
            position: None,
            field: None,
        }
    }

    pub fn unknown_command(command: &str) -> Self {
        Self::new("unknown-command", format!("unknown command `{command}`"))
    }

    pub fn bad_request(error: impl Display) -> Self {
        Self::new("bad-request", error.to_string())
    }
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        let message = error.to_string();
        match error {
            ApplicationError::InvalidExpression(parse_error) => Self {
                position: Some(parse_error.position),
                ..Self::new("syntax", message)
            },
            ApplicationError::Math(domain_error) => domain_error.into(),
            ApplicationError::InvalidField { field, cause } => Self {
                field: Some(field),
                ..Self::from(*cause)
            },
            ApplicationError::InvalidRequest(reason) => Self::new("invalid-request", reason),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        Self::new(domain_code(&error), error.to_string())
    }
}

fn domain_code(error: &DomainError) -> &'static str {
    match error {
        DomainError::UnknownVariable(_) => "unknown-variable",
        DomainError::DivisionByZero => "division-by-zero",
        DomainError::UndefinedResult => "undefined-result",
        DomainError::InvalidInterval { .. } => "invalid-interval",
        DomainError::InvalidSampleCount(_) => "invalid-sample-count",
        DomainError::NotIntegrable => "not-integrable",
        DomainError::ZeroVector => "zero-vector",
        DomainError::DimensionMismatch { .. } => "dimension-mismatch",
        DomainError::SingularMatrix => "singular-matrix",
        DomainError::DegenerateGeometry => "degenerate-geometry",
    }
}
