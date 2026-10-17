use std::collections::BTreeSet;

/// A validation failure at a canonical document field path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldError {
    /// Canonical field path, including collection indices when applicable.
    pub field: String,
    /// Human-readable explanation of the violated invariant.
    pub message: String,
}

/// Checks the canonical hyphenated UUID representation, version 7 and RFC variant.
pub fn is_uuid_v7(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 36
        && bytes.iter().enumerate().all(|(index, byte)| {
            if [8, 13, 18, 23].contains(&index) {
                *byte == b'-'
            } else {
                byte.is_ascii_hexdigit()
            }
        })
        && bytes[14] == b'7'
        && matches!(bytes[19], b'8' | b'9' | b'a' | b'A' | b'b' | b'B')
}

#[derive(Default)]
pub(crate) struct Validation(Vec<FieldError>);

impl Validation {
    pub(crate) fn require(&mut self, condition: bool, field: &str, message: &str) {
        if !condition {
            self.0.push(FieldError {
                field: field.into(),
                message: message.into(),
            });
        }
    }

    pub(crate) fn text(&mut self, value: &str, field: &str) {
        self.require(!value.trim().is_empty(), field, "Must not be blank");
    }

    pub(crate) fn optional_text(&mut self, value: &Option<String>, field: &str) {
        if let Some(value) = value {
            self.text(value, field);
        }
    }

    pub(crate) fn uuid(&mut self, value: &str, field: &str) {
        self.require(is_uuid_v7(value), field, "Must be a UUIDv7");
    }

    pub(crate) fn texts(&mut self, values: &[String], field: &str) {
        for (index, value) in values.iter().enumerate() {
            self.text(value, &format!("{field}[{index}]"));
        }
    }

    pub(crate) fn unique<'a>(&mut self, values: impl IntoIterator<Item = &'a str>, field: &str) {
        let mut seen = BTreeSet::new();
        for (index, value) in values.into_iter().enumerate() {
            self.require(
                seen.insert(value),
                &format!("{field}[{index}]"),
                "Duplicate value",
            );
        }
    }

    pub(crate) fn unique_uuids<'a>(
        &mut self,
        values: impl IntoIterator<Item = &'a str>,
        field: &str,
    ) {
        let mut seen = BTreeSet::new();
        for (index, value) in values.into_iter().enumerate() {
            self.require(
                seen.insert(value.to_ascii_lowercase()),
                &format!("{field}[{index}]"),
                "Duplicate UUID",
            );
        }
    }

    pub(crate) fn nested(&mut self, result: Result<(), Vec<FieldError>>, prefix: &str) {
        if let Err(errors) = result {
            self.0.extend(errors.into_iter().map(|mut error| {
                error.field = if prefix.is_empty() {
                    error.field
                } else if error.field.is_empty() {
                    prefix.to_owned()
                } else if error.field.starts_with('[') {
                    format!("{prefix}{}", error.field)
                } else {
                    format!("{prefix}.{}", error.field)
                };
                error
            }));
        }
    }

    pub(crate) fn finish(self) -> Result<(), Vec<FieldError>> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(self.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uuid_requires_canonical_shape_version_and_variant() {
        assert!(is_uuid_v7("0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90"));
        for invalid in [
            "",
            "0192f7a46c1e7b3a9d2e5f8a1c3b7e90",
            "0192f7a4-6c1e-4b3a-9d2e-5f8a1c3b7e90",
            "0192f7a4-6c1e-7b3a-7d2e-5f8a1c3b7e90",
            "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e9z",
        ] {
            assert!(!is_uuid_v7(invalid), "{invalid}");
        }
    }
}
