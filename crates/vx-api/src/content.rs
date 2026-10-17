//! Canonical content DTOs. Mathematical answer specifications belong to assessment.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::assessment::AnswerSpec;
pub use crate::content_decode::{decode_exercise, decode_list, decode_manifest, decode_pack};

/// Publication boundary shared by exercises, lists and packs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    Private,
}

/// Provenance of the exercise text, not of its references.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    Authored,
    Imported,
    Adapted,
}

/// Content license and attribution metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub kind: SourceKind,
    #[ts(optional = nullable)]
    pub citation: Option<String>,
    pub license: String,
    #[serde(default)]
    pub authors: Vec<String>,
}

/// Reference metadata kind; no file attachment is supported.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceType {
    Paper,
    Book,
    Notes,
    Video,
    Webpage,
}

/// External study reference; material links are reserved for F2 and must remain null.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    #[serde(rename = "type")]
    pub kind: ReferenceType,
    #[schemars(length(min = 1))]
    pub title: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[ts(optional = nullable)]
    #[schemars(regex(pattern = r"^[hH][tT][tT][pP][sS]?://"))]
    pub url: Option<String>,
    #[ts(optional = nullable)]
    pub doi: Option<String>,
    #[ts(optional = nullable)]
    pub arxiv: Option<String>,
    #[ts(optional = nullable)]
    pub isbn: Option<String>,
    #[ts(optional = nullable)]
    pub locator: Option<String>,
    #[schemars(length(min = 1))]
    pub license: String,
    #[ts(optional)]
    pub material: Option<()>,
}

/// Asset location inside a pack. Sanitization occurs at import/render time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub id: String,
    pub path: String,
    pub alt: String,
}

/// Request to the existing engine; not evidence of answer correctness.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct EngineRequest {
    pub command: String,
    pub request: serde_json::Value,
}

/// Optional authored steps or engine-produced explanation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Solution {
    #[serde(default)]
    pub steps: Vec<String>,
    #[ts(optional = nullable)]
    pub engine: Option<EngineRequest>,
}

/// Visualization registry key and parameters; registry validation belongs to F1.6.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Visualization {
    pub id: String,
    pub params: serde_json::Value,
}

/// A stable exercise part; verification fields arrive in F1.4.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Part {
    pub label: String,
    pub prompt: String,
    pub answer: AnswerSpec,
    #[serde(default)]
    pub hints: Vec<String>,
    #[ts(optional = nullable)]
    pub solution: Option<Solution>,
    #[ts(optional = nullable)]
    pub viz: Option<Visualization>,
    #[ts(optional = nullable)]
    pub points: Option<f64>,
}

/// Canonical revision of an exercise.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Exercise {
    #[schemars(range(min = 1, max = 1))]
    pub schema_version: u32,
    #[schemars(regex(
        pattern = r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-7[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$"
    ))]
    pub id: String,
    #[schemars(range(min = 1))]
    pub revision: u32,
    pub locale: String,
    pub title: String,
    pub statement: String,
    #[schemars(length(min = 1))]
    pub parts: Vec<Part>,
    #[schemars(length(min = 1))]
    pub skills: Vec<String>,
    #[schemars(range(min = 1, max = 5))]
    #[ts(optional = nullable)]
    pub difficulty: Option<u8>,
    #[ts(optional = nullable)]
    #[schemars(range(min = 1))]
    pub estimated_minutes: Option<u32>,
    #[serde(default)]
    pub assets: Vec<Asset>,
    pub source: Source,
    #[serde(default)]
    pub references: Vec<Reference>,
    pub visibility: Visibility,
    #[ts(optional)]
    pub params: Option<()>,
}

/// Ordered reference to an exercise and optionally some of its parts.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct ListItem {
    pub exercise: String,
    pub number: String,
    #[ts(optional = nullable)]
    pub parts: Option<Vec<String>>,
}

/// Canonical exercise collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExerciseList {
    #[schemars(range(min = 1, max = 1))]
    pub schema_version: u32,
    #[schemars(regex(
        pattern = r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-7[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$"
    ))]
    pub id: String,
    pub title: String,
    #[ts(optional = nullable)]
    pub description: Option<String>,
    pub items: Vec<ListItem>,
    pub source: Source,
    pub visibility: Visibility,
}

/// Metadata stored as manifest.json; list IDs are not paths.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackManifest {
    #[schemars(range(min = 1, max = 1))]
    pub schema_version: u32,
    #[schemars(regex(
        pattern = r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-7[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$"
    ))]
    pub id: String,
    pub name: String,
    #[ts(optional = nullable)]
    pub description: Option<String>,
    pub license: String,
    pub visibility: Visibility,
    pub lists: Vec<String>,
}

/// Decoded pack documents, independent of ZIP transport.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Pack {
    pub manifest: PackManifest,
    pub exercises: Vec<Exercise>,
    pub lists: Vec<ExerciseList>,
}
