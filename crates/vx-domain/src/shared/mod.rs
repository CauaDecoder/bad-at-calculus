//! Small values shared by the content and assessment contexts.

mod json;
mod validation;
mod values;

pub use json::JsonValue;
pub use validation::{is_uuid_v7, FieldError};
pub use values::{License, SkillId, Visibility};

pub(crate) use validation::Validation;
