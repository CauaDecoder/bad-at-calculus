use std::fmt;

/// Errors that come from breaking a mathematical rule, not from I/O or parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    UnknownVariable(String),
    DivisionByZero,
    /// The result is `NaN` or infinite, e.g. `ln(-1)` or `0^-1`.
    UndefinedResult,
    InvalidInterval {
        start: f64,
        end: f64,
    },
    InvalidSampleCount(usize),
    /// The integral does not converge or `f` is undefined inside the interval.
    NotIntegrable,
    /// Operation needs a non-zero vector (angle, unit vector, projection).
    ZeroVector,
    DimensionMismatch {
        expected: String,
        found: String,
    },
    SingularMatrix,
    /// Points that should define a plane or line are collinear or coincident.
    DegenerateGeometry,
}

impl fmt::Display for DomainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownVariable(name) => write!(formatter, "unknown variable `{name}`"),
            Self::DivisionByZero => write!(formatter, "division by zero"),
            Self::UndefinedResult => write!(formatter, "result is not a finite real number"),
            Self::InvalidInterval { start, end } => {
                write!(formatter, "invalid interval [{start}, {end}]")
            }
            Self::InvalidSampleCount(count) => write!(formatter, "invalid sample count {count}"),
            Self::NotIntegrable => write!(formatter, "the integral does not converge"),
            Self::ZeroVector => write!(formatter, "the zero vector has no direction"),
            Self::DimensionMismatch { expected, found } => {
                write!(formatter, "expected dimensions {expected}, found {found}")
            }
            Self::SingularMatrix => write!(formatter, "the matrix is singular"),
            Self::DegenerateGeometry => write!(formatter, "the points are collinear or coincident"),
        }
    }
}

impl std::error::Error for DomainError {}
