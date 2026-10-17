//! All F1 answer kinds; mathematical rules remain in the assessment domain.

mod grading;
mod input;

use input::{equation, feedback_answer, issue};
use std::collections::BTreeMap;

use vx_domain::{
    assessment::{
        compare_expressions, compare_numbers, AnswerOption, AnswerSpec, Comparison,
        EquationAnswerSpec, ExpressionAnswerSpec, ExpressionForm, SamplingDomain, SamplingSettings,
        ScalarAnswerSpec, Tolerance, Verdict,
    },
    Expr,
};

use crate::{ports::ExpressionParser, ApplicationError};

const MAX_SOURCE_BYTES: usize = 4096;
const MAX_SIGNIFICANT_CHARACTERS: usize = 128;
const MAX_VARIABLES: usize = 8;
const MAX_FEEDBACK_ENTRIES: usize = 16;
const DEFAULT_INTERVAL: [f64; 2] = [-10.0, 10.0];

/// Independent of content entities; callers provide the exercise identity for the seed.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckAnswerRequest {
    pub exercise_id: String,
    pub answer: AnswerSpec,
    pub response: String,
}

/// Student input failure, separate from an incorrect mathematical answer.
#[derive(Debug, Clone, PartialEq)]
pub struct InputIssue {
    pub field: String,
    pub message: String,
    pub position: Option<usize>,
}

/// Exactly one of comparison and input_issue is present; input failures are not attempts.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckAnswerReport {
    pub comparison: Option<Comparison>,
    pub input_issue: Option<InputIssue>,
    pub feedback: Option<String>,
}

/// Parses through a port and delegates mathematical equivalence to the domain.
pub struct CheckAnswer<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> CheckAnswer<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(
        &self,
        request: &CheckAnswerRequest,
    ) -> Result<CheckAnswerReport, ApplicationError> {
        validate_request(request)?;
        let compiled = self.compile(request)?;
        let feedback = self.compile_feedback(request)?;
        let comparison = match compiled.check(&self.parser, &request.response) {
            Ok(comparison) => comparison,
            Err(issue) => {
                return Ok(CheckAnswerReport {
                    comparison: None,
                    input_issue: Some(issue),
                    feedback: None,
                })
            }
        };
        let message = if comparison.verdict == Verdict::Incorrect {
            feedback.iter().find_map(|(wrong, message)| {
                wrong
                    .check(&self.parser, &request.response)
                    .ok()
                    .filter(|comparison| comparison.verdict == Verdict::Correct)
                    .map(|_| message.clone())
            })
        } else {
            None
        };
        Ok(CheckAnswerReport {
            comparison: Some(comparison),
            input_issue: None,
            feedback: message,
        })
    }

    fn compile(&self, request: &CheckAnswerRequest) -> Result<CompiledAnswer, ApplicationError> {
        match &request.answer {
            AnswerSpec::Numeric(spec) => {
                let expression =
                    authored_expression(&self.parser, &spec.value, &[], "answer.value")?;
                let value = expression.evaluate_constant().map_err(|error| {
                    ApplicationError::InvalidRequest(error.to_string()).in_field("answer.value")
                })?;
                Ok(CompiledAnswer::Numeric(
                    value,
                    spec.tolerance.clone().unwrap_or_default(),
                    spec.unit.clone(),
                ))
            }
            AnswerSpec::Expression(spec) => self.compile_expression(spec, &request.exercise_id),
            AnswerSpec::Equation(spec) => self.compile_equation(spec, &request.exercise_id),
            AnswerSpec::Set { values, .. } => Ok(CompiledAnswer::Set(self.compile_values(
                values,
                &request.exercise_id,
                "answer.values",
            )?)),
            AnswerSpec::Tuple { values, .. } => Ok(CompiledAnswer::Tuple(self.compile_values(
                values,
                &request.exercise_id,
                "answer.values",
            )?)),
            AnswerSpec::Matrix { rows, .. } => {
                if rows.iter().map(Vec::len).sum::<usize>() > 64 {
                    return Err(author_error(
                        "answer.rows",
                        "At most 64 matrix elements can be graded",
                    ));
                }
                let compiled = rows
                    .iter()
                    .enumerate()
                    .map(|(index, row)| {
                        self.compile_values(
                            row,
                            &request.exercise_id,
                            &format!("answer.rows[{index}]"),
                        )
                    })
                    .collect::<Result<_, _>>()?;
                Ok(CompiledAnswer::Matrix(compiled))
            }
            AnswerSpec::Choice {
                options,
                correct,
                multiple,
                ..
            } => Ok(CompiledAnswer::Choice {
                options: options.clone(),
                correct: correct.clone(),
                multiple: *multiple,
            }),
            AnswerSpec::Matching {
                left, right, pairs, ..
            } => Ok(CompiledAnswer::Matching {
                left: left.clone(),
                right: right.clone(),
                pairs: pairs.clone(),
            }),
            AnswerSpec::SelfGraded { .. } => Ok(CompiledAnswer::SelfGraded),
        }
    }

    fn compile_expression(
        &self,
        spec: &ExpressionAnswerSpec,
        identity: &str,
    ) -> Result<CompiledAnswer, ApplicationError> {
        if spec.variables.len() > MAX_VARIABLES {
            return Err(ApplicationError::InvalidRequest(
                "At most 8 variables can be sampled".into(),
            )
            .in_field("answer.variables"));
        }
        let expected = authored_expression(
            &self.parser,
            &spec.expected,
            &spec.variables,
            "answer.expected",
        )?;
        let settings = SamplingSettings::new(
            seed_for(identity),
            effective_domain(spec),
            spec.samples.unwrap_or(12),
            spec.tolerance.clone().unwrap_or_default(),
        )
        .ok_or_else(|| {
            ApplicationError::InvalidRequest("Grading supports 12 to 256 samples".into())
                .in_field("answer.samples")
        })?;
        Ok(CompiledAnswer::Expression {
            expected,
            variables: spec.variables.clone(),
            settings,
            form: spec.form,
        })
    }

    fn compile_equation(
        &self,
        spec: &EquationAnswerSpec,
        identity: &str,
    ) -> Result<CompiledAnswer, ApplicationError> {
        let residual = equation(&self.parser, &spec.expected, &spec.variables)
            .map_err(|issue| author_error("answer.expected", &issue.message))?;
        let template = ExpressionAnswerSpec {
            expected: "0".into(),
            variables: spec.variables.clone(),
            domain: spec.domain.clone(),
            samples: spec.samples,
            tolerance: spec.tolerance.clone(),
            form: None,
            feedback: vec![],
        };
        let CompiledAnswer::Expression {
            variables,
            settings,
            ..
        } = self.compile_expression(&template, identity)?
        else {
            return Err(author_error(
                "answer",
                "Expected expression sampling settings",
            ));
        };
        Ok(CompiledAnswer::Equation {
            expected: residual,
            variables,
            settings,
        })
    }

    fn compile_values(
        &self,
        values: &[ScalarAnswerSpec],
        identity: &str,
        field: &str,
    ) -> Result<Vec<CompiledAnswer>, ApplicationError> {
        if values.len() > 32 {
            return Err(author_error(
                field,
                "At most 32 elements per collection or row",
            ));
        }
        values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let answer = match value {
                    ScalarAnswerSpec::Numeric(spec) => AnswerSpec::Numeric(spec.clone()),
                    ScalarAnswerSpec::Expression(spec) => AnswerSpec::Expression(spec.clone()),
                };
                let request = CheckAnswerRequest {
                    exercise_id: identity.into(),
                    answer,
                    response: String::new(),
                };
                // Validate nested feedback too, even though this command reports parent-level feedback.
                self.compile_feedback(&request)
                    .map_err(|error| nested_error(error, &format!("{field}[{index}]")))?;
                self.compile(&request)
                    .map_err(|error| nested_error(error, &format!("{field}[{index}]")))
            })
            .collect()
    }

    fn compile_feedback(
        &self,
        request: &CheckAnswerRequest,
    ) -> Result<Vec<(CompiledAnswer, String)>, ApplicationError> {
        let items = feedback_items(&request.answer);
        if matches!(request.answer, AnswerSpec::SelfGraded { .. }) {
            return Ok(Vec::new());
        }
        if items.len() > MAX_FEEDBACK_ENTRIES {
            return Err(ApplicationError::InvalidRequest(
                "At most 16 feedback entries can be graded".into(),
            )
            .in_field("answer.feedback"));
        }
        items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let mut wrong = request.clone();
                wrong.answer =
                    feedback_answer(&request.answer, &item.matches).map_err(|issue| {
                        author_error(&format!("answer.feedback[{index}].matches"), &issue.message)
                    })?;
                let compiled = self.compile(&wrong).map_err(|error| {
                    ApplicationError::InvalidRequest(error.to_string())
                        .in_field(format!("answer.feedback[{index}].matches"))
                })?;
                Ok((compiled, item.message.clone()))
            })
            .collect()
    }
}

enum CompiledAnswer {
    Numeric(f64, Tolerance, Option<String>),
    Expression {
        expected: Expr,
        variables: Vec<String>,
        settings: SamplingSettings,
        form: Option<ExpressionForm>,
    },
    Equation {
        expected: Expr,
        variables: Vec<String>,
        settings: SamplingSettings,
    },
    Set(Vec<CompiledAnswer>),
    Tuple(Vec<CompiledAnswer>),
    Matrix(Vec<Vec<CompiledAnswer>>),
    Choice {
        options: Vec<AnswerOption>,
        correct: Vec<String>,
        multiple: bool,
    },
    Matching {
        left: Vec<AnswerOption>,
        right: Vec<AnswerOption>,
        pairs: BTreeMap<String, String>,
    },
    SelfGraded,
}

impl CompiledAnswer {
    fn compare(&self, response: &Expr) -> Comparison {
        match self {
            Self::Numeric(value, tolerance, _) => response.evaluate_constant().map_or(
                Comparison {
                    verdict: Verdict::Incorrect,
                    sampling: None,
                },
                |answer| compare_numbers(*value, answer, tolerance),
            ),
            Self::Expression {
                expected,
                settings,
                form,
                ..
            } => {
                let mut comparison = compare_expressions(expected, response, settings);
                if comparison.verdict == Verdict::Correct {
                    if let Some(form) = form {
                        comparison.verdict = vx_domain::assessment::check_form(response, *form);
                    }
                }
                comparison
            }
            _ => Comparison {
                verdict: Verdict::Indeterminate,
                sampling: None,
            },
        }
    }
}

fn validate_request(request: &CheckAnswerRequest) -> Result<(), ApplicationError> {
    if request.exercise_id.trim().is_empty() || request.exercise_id.len() > 128 {
        return Err(ApplicationError::InvalidRequest(
            "Exercise identity must contain 1 to 128 bytes".into(),
        )
        .in_field("exerciseId"));
    }
    validate_runtime_limits(&request.answer)?;
    request.answer.validate().map_err(|errors| {
        errors.first().map_or_else(
            || {
                ApplicationError::InvalidRequest("Invalid answer specification".into())
                    .in_field("answer")
            },
            |error| {
                ApplicationError::InvalidRequest(error.message.clone())
                    .in_field(format!("answer.{}", error.field))
            },
        )
    })
}

fn validate_runtime_limits(answer: &AnswerSpec) -> Result<(), ApplicationError> {
    match answer {
        AnswerSpec::Matrix { rows, .. } if rows.len() > 32 => Err(author_error(
            "answer.rows",
            "At most 32 matrix rows can be graded",
        )),
        AnswerSpec::Choice { options, .. } => validate_option_limits(options, "answer.options"),
        AnswerSpec::Matching { left, right, .. } => {
            validate_option_limits(left, "answer.left")?;
            validate_option_limits(right, "answer.right")
        }
        _ => Ok(()),
    }
}

fn validate_option_limits(options: &[AnswerOption], field: &str) -> Result<(), ApplicationError> {
    if options.len() > 32 {
        return Err(author_error(
            field,
            "At most 32 options per side can be graded",
        ));
    }
    for (index, option) in options.iter().enumerate() {
        if option.id.len() > 512 {
            return Err(author_error(
                &format!("{field}[{index}].id"),
                "Option identifiers must not exceed 512 bytes",
            ));
        }
    }
    Ok(())
}

fn effective_domain(spec: &ExpressionAnswerSpec) -> SamplingDomain {
    spec.variables
        .iter()
        .map(|name| {
            let bounds = spec
                .domain
                .as_ref()
                .and_then(|domain| domain.get(name))
                .copied()
                .unwrap_or(DEFAULT_INTERVAL);
            (name.clone(), bounds)
        })
        .collect()
}

fn numeric_source<'a>(source: &'a str, unit: &Option<String>) -> &'a str {
    if let Some(unit) = unit {
        if let Some(value) = source.trim_end().strip_suffix(unit) {
            if value.chars().last().is_some_and(char::is_whitespace) {
                return value.trim_end();
            }
        }
    }
    source
}

fn authored_expression(
    parser: &impl ExpressionParser,
    source: &str,
    variables: &[String],
    field: &str,
) -> Result<Expr, ApplicationError> {
    parse_checked(parser, source, variables)
        .map_err(|issue| ApplicationError::InvalidRequest(issue.message).in_field(field))
}

fn parse_checked(
    parser: &impl ExpressionParser,
    source: &str,
    variables: &[String],
) -> Result<Expr, InputIssue> {
    guard_source(source)?;
    let expression = parser.parse(source).map_err(|error| InputIssue {
        field: "response".into(),
        message: error.message,
        position: Some(error.position),
    })?;
    if let Some(name) = unknown_variable(&expression, variables) {
        return Err(InputIssue {
            field: "response".into(),
            message: format!("Undeclared variable `{name}`"),
            position: None,
        });
    }
    Ok(expression)
}

fn guard_source(source: &str) -> Result<(), InputIssue> {
    // Bound work before invoking an arbitrary parser port, including recursive Pratt syntax.
    if source.len() > MAX_SOURCE_BYTES {
        return Err(InputIssue {
            field: "response".into(),
            message: "Input exceeds 4096 bytes".into(),
            position: Some(0),
        });
    }
    let mut significant = 0;
    let mut braces: usize = 0;
    for (position, character) in source.chars().enumerate() {
        significant += usize::from(!character.is_whitespace());
        if significant > MAX_SIGNIFICANT_CHARACTERS {
            return Err(InputIssue {
                field: "response".into(),
                message: "Input exceeds 128 non-whitespace characters".into(),
                position: Some(position),
            });
        }
        match character {
            '{' => braces += 1,
            '}' => braces = braces.saturating_sub(1),
            '=' if braces == 0 => {
                return Err(InputIssue {
                    field: "response".into(),
                    message: "Enter an expression, not a definition or equation".into(),
                    position: Some(position),
                })
            }
            _ => {}
        }
    }
    Ok(())
}

fn unknown_variable<'a>(expression: &'a Expr, variables: &[String]) -> Option<&'a str> {
    match expression {
        Expr::Variable(name) => (!variables.contains(name)).then_some(name.as_str()),
        Expr::Negate(operand)
        | Expr::Call {
            argument: operand, ..
        } => unknown_variable(operand, variables),
        Expr::Binary { left, right, .. } => {
            unknown_variable(left, variables).or_else(|| unknown_variable(right, variables))
        }
        Expr::Piecewise(branches) => branches.iter().find_map(|branch| {
            branch
                .condition
                .as_ref()
                .and_then(|condition| {
                    condition.comparisons.iter().find_map(|comparison| {
                        unknown_variable(&comparison.left, variables)
                            .or_else(|| unknown_variable(&comparison.right, variables))
                    })
                })
                .or_else(|| unknown_variable(&branch.value, variables))
        }),
        _ => None,
    }
}

fn seed_for(identity: &str) -> u32 {
    // FNV-1a is specified byte-for-byte; UUID hexadecimal case does not change identity.
    identity.bytes().fold(2_166_136_261, |hash, byte| {
        (hash ^ u32::from(byte.to_ascii_lowercase())).wrapping_mul(16_777_619)
    })
}

fn author_error(field: &str, message: &str) -> ApplicationError {
    ApplicationError::InvalidRequest(message.into()).in_field(field)
}

fn nested_error(error: ApplicationError, prefix: &str) -> ApplicationError {
    match error {
        ApplicationError::InvalidField { field, cause } => (*cause).in_field(format!(
            "{prefix}{}",
            field.strip_prefix("answer").unwrap_or(&field)
        )),
        other => other.in_field(prefix),
    }
}

fn feedback_items(answer: &AnswerSpec) -> &[vx_domain::assessment::Feedback] {
    match answer {
        AnswerSpec::Numeric(spec) => &spec.feedback,
        AnswerSpec::Expression(spec) => &spec.feedback,
        AnswerSpec::Equation(spec) => &spec.feedback,
        AnswerSpec::Set { feedback, .. }
        | AnswerSpec::Tuple { feedback, .. }
        | AnswerSpec::Matrix { feedback, .. }
        | AnswerSpec::Choice { feedback, .. }
        | AnswerSpec::Matching { feedback, .. }
        | AnswerSpec::SelfGraded { feedback, .. } => feedback,
    }
}

#[cfg(test)]
mod tests;
