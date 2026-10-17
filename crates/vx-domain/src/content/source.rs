use crate::shared::{FieldError, License, Validation};

/// The editorial origin of a document, distinct from its distribution permission.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    /// Original content written by the credited authors.
    Authored,
    /// Content imported from an identified source.
    Imported,
    /// Content adapted from an identified source.
    Adapted,
}

/// Provenance and rights of an exercise or list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// How the content was produced.
    pub kind: SourceKind,
    /// Source attribution, mandatory for imported and adapted content.
    pub citation: Option<String>,
    /// Rights of this content, not those of linked references.
    pub license: License,
    /// Credited authors; original content requires at least one.
    pub authors: Vec<String>,
}

impl Source {
    /// Validates attribution and the presence of rights metadata.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.nested(self.license.validate(), "license");
        validation.optional_text(&self.citation, "citation");
        validation.texts(&self.authors, "authors");
        match self.kind {
            SourceKind::Authored => validation.require(
                !self.authors.is_empty(),
                "authors",
                "Authored content requires an author",
            ),
            SourceKind::Imported | SourceKind::Adapted => validation.require(
                self.citation
                    .as_ref()
                    .is_some_and(|citation| !citation.trim().is_empty()),
                "citation",
                "Imported or adapted content requires attribution",
            ),
        }
        validation.finish()
    }

    /// Applies the public bank's authoring and open-license policy (ADR 0012).
    pub fn validate_for_publication(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.nested(self.validate(), "");
        let compatible = match self.kind {
            SourceKind::Authored => self.license == License::CcBySa40,
            SourceKind::Imported | SourceKind::Adapted => self.license.allows_publication(),
        };
        validation.require(
            compatible,
            "license",
            "License is incompatible with public content",
        );
        validation.finish()
    }
}
