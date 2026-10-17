use super::{FieldError, Validation};

/// A dotted taxonomy identifier; existence in a catalog is checked by application code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkillId(pub String);

impl SkillId {
    /// Checks nonempty lowercase, kebab-case taxonomy segments.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        let valid = self.0.split('.').all(|segment| {
            !segment.is_empty()
                && segment.split('-').all(|word| !word.is_empty())
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        });
        validation.require(valid, "", "Must be a dotted taxonomy identifier");
        validation.finish()
    }
}

/// Rights attached to an edition or content document, not to application code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum License {
    /// Creative Commons Attribution ShareAlike 4.0.
    CcBySa40,
    /// Creative Commons Attribution 4.0.
    CcBy40,
    /// Creative Commons Zero 1.0.
    Cc0,
    /// Copyrighted material without public redistribution permission.
    Proprietary,
    /// Rights have not been established.
    Unknown,
    /// Other identifiers are preserved, but never presumed publication-compatible.
    Other(String),
}

impl License {
    /// Returns the canonical license identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::CcBySa40 => "CC-BY-SA-4.0",
            Self::CcBy40 => "CC-BY-4.0",
            Self::Cc0 => "CC0-1.0",
            Self::Proprietary => "proprietary",
            Self::Unknown => "unknown",
            Self::Other(value) => value,
        }
    }

    /// Allows only the explicitly reviewed open licenses for public documents.
    pub fn allows_publication(&self) -> bool {
        matches!(self, Self::CcBySa40 | Self::CcBy40 | Self::Cc0)
    }

    /// Checks that a license identifier was supplied, without granting permission.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(self.as_str(), "");
        validation.finish()
    }
}

impl From<&str> for License {
    fn from(value: &str) -> Self {
        match value {
            "CC-BY-SA-4.0" => Self::CcBySa40,
            "CC-BY-4.0" => Self::CcBy40,
            "CC0-1.0" => Self::Cc0,
            "proprietary" => Self::Proprietary,
            "unknown" => Self::Unknown,
            other => Self::Other(other.into()),
        }
    }
}

/// A document's distribution boundary; public containers never override it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// Eligible for public distribution if its own rights permit it.
    Public,
    /// Local private content, never eligible for public distribution.
    Private,
}
