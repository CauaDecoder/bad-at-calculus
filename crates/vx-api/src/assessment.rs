//! Serializable answer specifications; executable F1.2 requests live in `assessment_check`.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Absolute and relative comparison tolerances.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Tolerance {
    #[ts(optional = nullable)]
    #[schemars(range(min = 0))]
    pub abs: Option<f64>,
    #[ts(optional = nullable)]
    #[schemars(range(min = 0))]
    pub rel: Option<f64>,
}

/// Feedback for a predictable incorrect answer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct Feedback {
    pub matches: String,
    pub message: String,
}

/// Required structural expression form.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum ExpressionForm {
    Factored,
    Expanded,
    NoNegativeExponents,
}

/// Constant expression and optional numeric comparison metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct NumericAnswer {
    #[schemars(length(min = 1))]
    pub value: String,
    #[ts(optional = nullable)]
    pub tolerance: Option<Tolerance>,
    #[ts(optional = nullable)]
    pub unit: Option<String>,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Expected expression and deterministic sampling settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct ExpressionAnswer {
    #[schemars(length(min = 1))]
    pub expected: String,
    #[schemars(length(min = 1))]
    pub variables: Vec<String>,
    #[ts(optional = nullable)]
    pub domain: Option<BTreeMap<String, [f64; 2]>>,
    #[ts(optional = nullable)]
    pub form: Option<ExpressionForm>,
    #[ts(optional = nullable)]
    #[schemars(range(min = 12))]
    pub samples: Option<u32>,
    #[ts(optional = nullable)]
    pub tolerance: Option<Tolerance>,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Equation represented as `lhs = rhs`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct EquationAnswer {
    #[schemars(length(min = 1))]
    pub expected: String,
    #[schemars(length(min = 1))]
    pub variables: Vec<String>,
    #[ts(optional = nullable)]
    pub domain: Option<BTreeMap<String, [f64; 2]>>,
    #[ts(optional = nullable)]
    #[schemars(range(min = 12))]
    pub samples: Option<u32>,
    #[ts(optional = nullable)]
    pub tolerance: Option<Tolerance>,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Nonrecursive scalar element of a collection (ADR 0016).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum ScalarAnswerSpec {
    Numeric(NumericAnswer),
    Expression(ExpressionAnswer),
}

/// Unordered collection; `ordered` must be false.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct SetAnswer {
    pub values: Vec<ScalarAnswerSpec>,
    #[ts(type = "false")]
    #[schemars(schema_with = "unordered_schema")]
    pub ordered: bool,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Ordered, nonempty collection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct TupleAnswer {
    #[schemars(length(min = 1))]
    pub values: Vec<ScalarAnswerSpec>,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Nonempty rectangular matrix of scalar specifications.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct MatrixAnswer {
    #[schemars(schema_with = "matrix_rows_schema")]
    pub rows: Vec<Vec<ScalarAnswerSpec>>,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Stable identity and display text of an objective option.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct AnswerOption {
    pub id: String,
    pub text: String,
}

/// Single or multiple choice specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct ChoiceAnswer {
    pub options: Vec<AnswerOption>,
    pub correct: Vec<String>,
    pub multiple: bool,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Matching specification with stable option identities.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct MatchingAnswer {
    pub left: Vec<AnswerOption>,
    pub right: Vec<AnswerOption>,
    pub pairs: BTreeMap<String, String>,
    #[serde(default)]
    pub partial_credit: bool,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// Manual comparison with a rubric, never automatic grading.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct SelfGradedAnswer {
    #[ts(optional = nullable)]
    pub rubric: Option<String>,
    #[serde(default)]
    pub feedback: Vec<Feedback>,
}

/// All nine F1 answer kinds. `code` and `sql` are intentionally unsupported.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum AnswerSpec {
    Numeric(NumericAnswer),
    Expression(ExpressionAnswer),
    Equation(EquationAnswer),
    Set(SetAnswer),
    Tuple(TupleAnswer),
    Matrix(MatrixAnswer),
    Choice(ChoiceAnswer),
    Matching(MatchingAnswer),
    SelfGraded(SelfGradedAnswer),
}

fn unordered_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    let mut schema = bool::json_schema(generator).into_object();
    schema.enum_values = Some(vec![serde_json::Value::Bool(false)]);
    schema.into()
}

fn matrix_rows_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    use schemars::schema::SingleOrVec;

    let mut row = Vec::<ScalarAnswerSpec>::json_schema(generator).into_object();
    row.array().min_items = Some(1);
    let mut rows = Vec::<Vec<ScalarAnswerSpec>>::json_schema(generator).into_object();
    rows.array().min_items = Some(1);
    rows.array().items = Some(SingleOrVec::Single(Box::new(row.into())));
    rows.into()
}
