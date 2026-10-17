use crate::shared::{FieldError, Validation, Visibility};

use super::Source;

/// A list entry selecting all or some parts of an exercise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    /// UUIDv7 of the referenced exercise.
    pub exercise: String,
    /// Editorial display number, not the exercise identity.
    pub number: String,
    /// Optional nonempty selection of distinct part labels; absence selects all parts.
    pub parts: Option<Vec<String>>,
}

/// An ordered exercise collection with its own provenance and visibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExerciseList {
    /// Canonical content schema version, currently 1.
    pub schema_version: u32,
    /// Stable UUIDv7 identity.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Optional Markdown introduction.
    pub description: Option<String>,
    /// Ordered exercise entries.
    pub items: Vec<ListItem>,
    /// Provenance and rights of the list itself.
    pub source: Source,
    /// Distribution boundary, independent of its exercises.
    pub visibility: Visibility,
}

impl ListItem {
    /// Checks local structure; resolving exercise and part references requires a pack.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.uuid(&self.exercise, "exercise");
        validation.text(&self.number, "number");
        if let Some(parts) = &self.parts {
            validation.require(
                !parts.is_empty(),
                "parts",
                "An explicit part selection must not be empty",
            );
            validation.texts(parts, "parts");
            validation.unique(parts.iter().map(String::as_str), "parts");
        }
        validation.finish()
    }
}

impl ExerciseList {
    /// Checks metadata, duplicates and each local entry without resolving external references.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.require(
            self.schema_version == 1,
            "schemaVersion",
            "Unsupported schema version",
        );
        validation.uuid(&self.id, "id");
        validation.text(&self.title, "title");
        validation.optional_text(&self.description, "description");
        validation.unique_uuids(
            self.items.iter().map(|item| item.exercise.as_str()),
            "items",
        );
        validation.unique(self.items.iter().map(|item| item.number.as_str()), "items");
        for (index, item) in self.items.iter().enumerate() {
            validation.nested(item.validate(), &format!("items[{index}]"));
        }
        let source_result = match self.visibility {
            Visibility::Public => self.source.validate_for_publication(),
            Visibility::Private => self.source.validate(),
        };
        validation.nested(source_result, "source");
        validation.finish()
    }

    /// Requires the list itself to be public and compatible with public distribution.
    pub fn validate_for_publication(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.nested(self.validate(), "");
        validation.require(
            self.visibility == Visibility::Public,
            "visibility",
            "Private content cannot be published",
        );
        if self.visibility == Visibility::Private {
            validation.nested(self.source.validate_for_publication(), "source");
        }
        validation.finish()
    }
}
