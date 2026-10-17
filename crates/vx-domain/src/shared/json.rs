use std::collections::BTreeMap;

use super::{FieldError, Validation};

/// Transport-independent JSON data for technical visualization and engine payloads.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// JSON null.
    Null,
    /// JSON boolean.
    Bool(bool),
    /// A signed integer, preserving engine fields that do not accept floating point.
    Integer(i64),
    /// A large unsigned integer, without rounding through floating point.
    Unsigned(u64),
    /// A floating-point JSON number; validation rejects NaN and infinities.
    Number(f64),
    /// JSON string.
    String(String),
    /// Ordered sequence of JSON values.
    Array(Vec<Self>),
    /// Deterministically ordered object members.
    Object(BTreeMap<String, Self>),
}

impl JsonValue {
    /// Checks that all numeric values can be represented as JSON.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        match self {
            Self::Number(value) => {
                validation.require(value.is_finite(), "", "JSON numbers must be finite")
            }
            Self::Array(values) => {
                for (index, value) in values.iter().enumerate() {
                    validation.nested(value.validate(), &format!("[{index}]"));
                }
            }
            Self::Object(values) => {
                for (key, value) in values {
                    validation.nested(value.validate(), key);
                }
            }
            _ => {}
        }
        validation.finish()
    }
}
