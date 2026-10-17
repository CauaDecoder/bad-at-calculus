use std::collections::BTreeMap;

use crate::shared::{FieldError, License, Validation, Visibility};

use super::{Exercise, ExerciseList};

/// Canonical pack metadata; its identity is the pack identity, not a separate entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackManifest {
    /// Canonical schema version, currently 1.
    pub schema_version: u32,
    /// Stable UUIDv7, preserved when the pack is renamed or reexported.
    pub id: String,
    /// Human-readable pack name.
    pub name: String,
    /// Optional Markdown description.
    pub description: Option<String>,
    /// Rights of the pack, never overriding document rights.
    pub license: License,
    /// Distribution boundary, never promoting private documents.
    pub visibility: Visibility,
    /// Ordered distinct UUIDv7 list identifiers; empty allows standalone exercises.
    pub lists: Vec<String>,
}

/// Included content and the manifest needed to resolve all cross-document references.
#[derive(Debug, Clone, PartialEq)]
pub struct Pack {
    /// Pack identity and distribution metadata.
    pub manifest: PackManifest,
    /// Included exercise documents, independent of ZIP filenames.
    pub exercises: Vec<Exercise>,
    /// Included list documents, independent of ZIP filenames.
    pub lists: Vec<ExerciseList>,
}

impl PackManifest {
    /// Validates identity, ordered list identifiers and declared public rights.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.require(
            self.schema_version == 1,
            "schemaVersion",
            "Unsupported schema version",
        );
        validation.uuid(&self.id, "id");
        validation.text(&self.name, "name");
        validation.optional_text(&self.description, "description");
        validation.nested(self.license.validate(), "license");
        if self.visibility == Visibility::Public {
            validation.require(
                self.license.allows_publication(),
                "license",
                "License is incompatible with public content",
            );
        }
        validation.unique_uuids(self.lists.iter().map(String::as_str), "lists");
        for (index, id) in self.lists.iter().enumerate() {
            validation.uuid(id, &format!("lists[{index}]"));
        }
        validation.finish()
    }

    /// Requires explicit public visibility and an approved open license.
    pub fn validate_for_publication(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.nested(self.validate(), "");
        validation.require(
            self.visibility == Visibility::Public,
            "visibility",
            "Private content cannot be published",
        );
        if self.visibility == Visibility::Private {
            validation.require(
                self.license.allows_publication(),
                "license",
                "License is incompatible with public content",
            );
        }
        validation.finish()
    }
}

impl Pack {
    /// Checks every document and resolves all list references; no archive I/O occurs here.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.nested(self.manifest.validate(), "manifest");
        validation.unique_uuids(
            self.exercises.iter().map(|exercise| exercise.id.as_str()),
            "exercises",
        );
        validation.unique_uuids(self.lists.iter().map(|list| list.id.as_str()), "lists");
        self.validate_documents(&mut validation);
        self.validate_references(&mut validation);
        validation.finish()
    }

    /// Refuses a private manifest or any included private or incompatible document.
    pub fn validate_for_publication(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.nested(self.validate(), "");
        if self.manifest.visibility == Visibility::Private {
            validation.nested(self.manifest.validate_for_publication(), "manifest");
            self.validate_public_documents(&mut validation);
        }
        validation.finish()
    }

    fn validate_documents(&self, validation: &mut Validation) {
        if self.manifest.visibility == Visibility::Public {
            self.validate_public_documents(validation);
            return;
        }
        for (index, exercise) in self.exercises.iter().enumerate() {
            validation.nested(exercise.validate(), &format!("exercises[{index}]"));
        }
        for (index, list) in self.lists.iter().enumerate() {
            validation.nested(list.validate(), &format!("lists[{index}]"));
        }
    }

    fn validate_public_documents(&self, validation: &mut Validation) {
        for (index, exercise) in self.exercises.iter().enumerate() {
            validation.nested(
                exercise.validate_for_publication(),
                &format!("exercises[{index}]"),
            );
        }
        for (index, list) in self.lists.iter().enumerate() {
            validation.nested(list.validate_for_publication(), &format!("lists[{index}]"));
        }
    }

    fn validate_references(&self, validation: &mut Validation) {
        let exercises: BTreeMap<_, _> = self
            .exercises
            .iter()
            .map(|exercise| (exercise.id.to_ascii_lowercase(), exercise))
            .collect();
        let lists: BTreeMap<_, _> = self
            .lists
            .iter()
            .map(|list| (list.id.to_ascii_lowercase(), list))
            .collect();
        for (index, id) in self.manifest.lists.iter().enumerate() {
            validation.require(
                lists.contains_key(&id.to_ascii_lowercase()),
                &format!("manifest.lists[{index}]"),
                "List is not included in the pack",
            );
        }
        for (list_index, list) in self.lists.iter().enumerate() {
            for (item_index, item) in list.items.iter().enumerate() {
                let prefix = format!("lists[{list_index}].items[{item_index}]");
                let Some(exercise) = exercises.get(&item.exercise.to_ascii_lowercase()) else {
                    validation.require(
                        false,
                        &format!("{prefix}.exercise"),
                        "Exercise is not included in the pack",
                    );
                    continue;
                };
                if let Some(parts) = &item.parts {
                    for (index, label) in parts.iter().enumerate() {
                        validation.require(
                            exercise.parts.iter().any(|part| part.label == *label),
                            &format!("{prefix}.parts[{index}]"),
                            "Part does not exist in the referenced exercise",
                        );
                    }
                }
            }
        }
    }
}
