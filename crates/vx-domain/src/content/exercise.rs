use crate::assessment::AnswerSpec;
use crate::shared::{FieldError, JsonValue, SkillId, Validation, Visibility};

use super::{Reference, Source};

/// A relative asset reference; file reading and SVG sanitization belong to adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// Stable identifier within the exercise.
    pub id: String,
    /// Relative path inside the pack's assets directory.
    pub path: String,
    /// Accessible description of the asset.
    pub alt: String,
}

/// An optional engine-generated solution request, not verification evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct EngineCall {
    /// Engine command identifier.
    pub command: String,
    /// Technical JSON payload; command-specific validation occurs at its boundary.
    pub request: JsonValue,
}

/// Authored solution steps and an optional engine request.
#[derive(Debug, Clone, PartialEq)]
pub struct Solution {
    /// Markdown solution steps in display order.
    pub steps: Vec<String>,
    /// Optional command for obtaining solution steps.
    pub engine: Option<EngineCall>,
}

/// A reference to the visualization registry and its technical parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct Visualization {
    /// Registry identifier, optionally including a registry version.
    pub id: String,
    /// Registry-specific JSON parameters.
    pub params: JsonValue,
}

/// The independently answerable editorial unit owned by content.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    /// Stable label, unique within the exercise.
    pub label: String,
    /// Markdown question text.
    pub prompt: String,
    /// Public assessment specification; no duplicate content-owned answer type.
    pub answer: AnswerSpec,
    /// Markdown hints in disclosure order.
    pub hints: Vec<String>,
    /// Optional authored or engine-assisted solution.
    pub solution: Option<Solution>,
    /// Optional laboratory visualization.
    pub viz: Option<Visualization>,
    /// Optional positive point weight.
    pub points: Option<f64>,
}

/// A canonical F1 exercise; future parameterization and evidence are deliberately absent.
#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    /// Canonical content schema version, currently 1.
    pub schema_version: u32,
    /// Stable UUIDv7 identity.
    pub id: String,
    /// Positive revision used to bind attempts to the viewed content.
    pub revision: u32,
    /// Content language tag.
    pub locale: String,
    /// Display title.
    pub title: String,
    /// Shared Markdown statement.
    pub statement: String,
    /// At least one independently answerable part.
    pub parts: Vec<Part>,
    /// Distinct taxonomy identifiers; catalog membership is checked by application code.
    pub skills: Vec<SkillId>,
    /// Optional difficulty from 1 to 5.
    pub difficulty: Option<u8>,
    /// Optional positive study-time estimate.
    pub estimated_minutes: Option<u32>,
    /// Referenced pack assets.
    pub assets: Vec<Asset>,
    /// Provenance and rights.
    pub source: Source,
    /// Linked study metadata, never attached third-party material.
    pub references: Vec<Reference>,
    /// Distribution boundary.
    pub visibility: Visibility,
}

impl Asset {
    /// Checks relative pack paths and accessible metadata, without reading files.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.id, "id");
        validation.text(&self.alt, "alt");
        let safe = self.path.starts_with("assets/")
            && !self.path.contains(['\\', ':'])
            && !self.path.chars().any(char::is_control)
            && self
                .path
                .split('/')
                .all(|segment| !segment.is_empty() && segment != "." && segment != "..");
        validation.require(
            safe,
            "path",
            "Must be a relative assets path without traversal",
        );
        validation.finish()
    }
}

impl EngineCall {
    /// Checks presence and JSON representability, without executing a command.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.command, "command");
        validation.nested(self.request.validate(), "request");
        validation.finish()
    }
}

impl Solution {
    /// Requires at least one authored step or an engine request.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.require(
            !self.steps.is_empty() || self.engine.is_some(),
            "steps",
            "Provide steps or an engine request",
        );
        validation.texts(&self.steps, "steps");
        if let Some(engine) = &self.engine {
            validation.nested(engine.validate(), "engine");
        }
        validation.finish()
    }
}

impl Visualization {
    /// Checks registry metadata; registry-specific schemas belong to the outer boundary.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.id, "id");
        validation.nested(self.params.validate(), "params");
        validation.finish()
    }
}

impl Part {
    /// Validates the editorial structure and delegates answer rules to assessment.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.label, "label");
        validation.text(&self.prompt, "prompt");
        validation.nested(self.answer.validate(), "answer");
        validation.texts(&self.hints, "hints");
        if let Some(solution) = &self.solution {
            validation.nested(solution.validate(), "solution");
        }
        if let Some(viz) = &self.viz {
            validation.nested(viz.validate(), "viz");
        }
        if let Some(points) = self.points {
            validation.require(
                points.is_finite() && points > 0.0,
                "points",
                "Must be finite and positive",
            );
        }
        validation.finish()
    }
}

impl Exercise {
    /// Validates canonical identity, metadata and parts, including declared public rights.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.require(
            self.schema_version == 1,
            "schemaVersion",
            "Unsupported schema version",
        );
        validation.uuid(&self.id, "id");
        validation.require(self.revision > 0, "revision", "Must be positive");
        validation.text(&self.locale, "locale");
        validation.text(&self.title, "title");
        validation.text(&self.statement, "statement");
        validation.require(
            !self.parts.is_empty(),
            "parts",
            "At least one part is required",
        );
        validation.unique(self.parts.iter().map(|part| part.label.as_str()), "parts");
        for (index, part) in self.parts.iter().enumerate() {
            validation.nested(part.validate(), &format!("parts[{index}]"));
        }
        self.validate_metadata(&mut validation);
        validation.finish()
    }

    /// Rejects private content even when all its other metadata is publication-compatible.
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

    fn validate_metadata(&self, validation: &mut Validation) {
        validation.require(
            !self.skills.is_empty(),
            "skills",
            "At least one skill is required",
        );
        validation.unique(self.skills.iter().map(|skill| skill.0.as_str()), "skills");
        for (index, skill) in self.skills.iter().enumerate() {
            validation.nested(skill.validate(), &format!("skills[{index}]"));
        }
        if let Some(difficulty) = self.difficulty {
            validation.require(
                (1..=5).contains(&difficulty),
                "difficulty",
                "Must be between 1 and 5",
            );
        }
        if let Some(minutes) = self.estimated_minutes {
            validation.require(minutes > 0, "estimatedMinutes", "Must be positive");
        }
        validation.unique(self.assets.iter().map(|asset| asset.id.as_str()), "assets");
        for (index, asset) in self.assets.iter().enumerate() {
            validation.nested(asset.validate(), &format!("assets[{index}]"));
        }
        let source_result = match self.visibility {
            Visibility::Public => self.source.validate_for_publication(),
            Visibility::Private => self.source.validate(),
        };
        validation.nested(source_result, "source");
        for (index, reference) in self.references.iter().enumerate() {
            validation.nested(reference.validate(), &format!("references[{index}]"));
        }
    }
}
