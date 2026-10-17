//! Transport for all nine F1 answer kinds.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use vx_application::use_cases::{CheckAnswer, CheckAnswerRequest};
use vx_domain::assessment::{SamplingReport, Verdict};
use vx_infra::parser::PrattParser;

use crate::{assessment::*, ApiError};

/// Only executable specifications are advertised by the grading request schema.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[schemars(deny_unknown_fields)]
pub enum GradableAnswer {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CheckRequest {
    pub exercise_id: String,
    pub answer: GradableAnswer,
    pub response: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "snake_case")]
pub enum CheckStatus {
    Correct,
    Incorrect,
    Indeterminate,
    InvalidInput,
    SelfGraded,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SamplingResult {
    pub seed: u32,
    pub domain: std::collections::BTreeMap<String, [f64; 2]>,
    pub requested: u32,
    pub valid: u32,
    pub discarded: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(deny_unknown_fields)]
pub struct InputError {
    pub field: String,
    pub message: String,
    pub position: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CheckResponse {
    pub status: CheckStatus,
    pub sampling: Option<SamplingResult>,
    pub feedback: Option<String>,
    pub input_error: Option<InputError>,
}

pub fn check(request: CheckRequest) -> Result<CheckResponse, ApiError> {
    let answer = match request.answer {
        GradableAnswer::Numeric(spec) => AnswerSpec::Numeric(spec),
        GradableAnswer::Expression(spec) => AnswerSpec::Expression(spec),
        GradableAnswer::Equation(spec) => AnswerSpec::Equation(spec),
        GradableAnswer::Set(spec) => AnswerSpec::Set(spec),
        GradableAnswer::Tuple(spec) => AnswerSpec::Tuple(spec),
        GradableAnswer::Matrix(spec) => AnswerSpec::Matrix(spec),
        GradableAnswer::Choice(spec) => AnswerSpec::Choice(spec),
        GradableAnswer::Matching(spec) => AnswerSpec::Matching(spec),
        GradableAnswer::SelfGraded(spec) => AnswerSpec::SelfGraded(spec),
    };
    let report = CheckAnswer::new(PrattParser).execute(&CheckAnswerRequest {
        exercise_id: request.exercise_id,
        answer: answer.into(),
        response: request.response,
    })?;
    let (status, sampling) =
        report
            .comparison
            .map_or((CheckStatus::InvalidInput, None), |comparison| {
                let status = match comparison.verdict {
                    Verdict::Correct => CheckStatus::Correct,
                    Verdict::Incorrect => CheckStatus::Incorrect,
                    Verdict::Indeterminate => CheckStatus::Indeterminate,
                    Verdict::SelfGraded => CheckStatus::SelfGraded,
                };
                (status, comparison.sampling.map(Into::into))
            });
    Ok(CheckResponse {
        status,
        sampling,
        feedback: report.feedback,
        input_error: report.input_issue.map(|issue| InputError {
            field: issue.field,
            message: issue.message,
            position: issue.position,
        }),
    })
}

impl From<SamplingReport> for SamplingResult {
    fn from(report: SamplingReport) -> Self {
        Self {
            seed: report.seed,
            domain: report.domain,
            requested: report.requested,
            valid: report.valid,
            discarded: report.discarded,
        }
    }
}
