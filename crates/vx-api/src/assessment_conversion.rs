//! Explicit transport conversions; mathematical validation stays in the domain.

use crate::assessment as dto;
use vx_domain::assessment as domain;

impl From<dto::Tolerance> for domain::Tolerance {
    fn from(value: dto::Tolerance) -> Self {
        Self {
            abs: value.abs,
            rel: value.rel,
        }
    }
}

impl From<domain::Tolerance> for dto::Tolerance {
    fn from(value: domain::Tolerance) -> Self {
        Self {
            abs: value.abs,
            rel: value.rel,
        }
    }
}

impl From<dto::Feedback> for domain::Feedback {
    fn from(value: dto::Feedback) -> Self {
        Self {
            matches: value.matches,
            message: value.message,
        }
    }
}

impl From<domain::Feedback> for dto::Feedback {
    fn from(value: domain::Feedback) -> Self {
        Self {
            matches: value.matches,
            message: value.message,
        }
    }
}

impl From<dto::ExpressionForm> for domain::ExpressionForm {
    fn from(value: dto::ExpressionForm) -> Self {
        match value {
            dto::ExpressionForm::Factored => Self::Factored,
            dto::ExpressionForm::Expanded => Self::Expanded,
            dto::ExpressionForm::NoNegativeExponents => Self::NoNegativeExponents,
        }
    }
}

impl From<domain::ExpressionForm> for dto::ExpressionForm {
    fn from(value: domain::ExpressionForm) -> Self {
        match value {
            domain::ExpressionForm::Factored => Self::Factored,
            domain::ExpressionForm::Expanded => Self::Expanded,
            domain::ExpressionForm::NoNegativeExponents => Self::NoNegativeExponents,
        }
    }
}

impl From<dto::NumericAnswer> for domain::NumericAnswerSpec {
    fn from(value: dto::NumericAnswer) -> Self {
        Self {
            value: value.value,
            tolerance: value.tolerance.map(Into::into),
            unit: value.unit,
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<domain::NumericAnswerSpec> for dto::NumericAnswer {
    fn from(value: domain::NumericAnswerSpec) -> Self {
        Self {
            value: value.value,
            tolerance: value.tolerance.map(Into::into),
            unit: value.unit,
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::ExpressionAnswer> for domain::ExpressionAnswerSpec {
    fn from(value: dto::ExpressionAnswer) -> Self {
        Self {
            expected: value.expected,
            variables: value.variables,
            domain: value.domain,
            form: value.form.map(Into::into),
            samples: value.samples,
            tolerance: value.tolerance.map(Into::into),
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<domain::ExpressionAnswerSpec> for dto::ExpressionAnswer {
    fn from(value: domain::ExpressionAnswerSpec) -> Self {
        Self {
            expected: value.expected,
            variables: value.variables,
            domain: value.domain,
            form: value.form.map(Into::into),
            samples: value.samples,
            tolerance: value.tolerance.map(Into::into),
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::EquationAnswer> for domain::EquationAnswerSpec {
    fn from(value: dto::EquationAnswer) -> Self {
        Self {
            expected: value.expected,
            variables: value.variables,
            domain: value.domain,
            samples: value.samples,
            tolerance: value.tolerance.map(Into::into),
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<domain::EquationAnswerSpec> for dto::EquationAnswer {
    fn from(value: domain::EquationAnswerSpec) -> Self {
        Self {
            expected: value.expected,
            variables: value.variables,
            domain: value.domain,
            samples: value.samples,
            tolerance: value.tolerance.map(Into::into),
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::ScalarAnswerSpec> for domain::ScalarAnswerSpec {
    fn from(value: dto::ScalarAnswerSpec) -> Self {
        match value {
            dto::ScalarAnswerSpec::Numeric(value) => Self::Numeric(value.into()),
            dto::ScalarAnswerSpec::Expression(value) => Self::Expression(value.into()),
        }
    }
}

impl From<domain::ScalarAnswerSpec> for dto::ScalarAnswerSpec {
    fn from(value: domain::ScalarAnswerSpec) -> Self {
        match value {
            domain::ScalarAnswerSpec::Numeric(value) => Self::Numeric(value.into()),
            domain::ScalarAnswerSpec::Expression(value) => Self::Expression(value.into()),
        }
    }
}

impl From<dto::AnswerOption> for domain::AnswerOption {
    fn from(value: dto::AnswerOption) -> Self {
        Self {
            id: value.id,
            text: value.text,
        }
    }
}

impl From<domain::AnswerOption> for dto::AnswerOption {
    fn from(value: domain::AnswerOption) -> Self {
        Self {
            id: value.id,
            text: value.text,
        }
    }
}

impl From<dto::SetAnswer> for domain::AnswerSpec {
    fn from(value: dto::SetAnswer) -> Self {
        Self::Set {
            values: value.values.into_iter().map(Into::into).collect(),
            ordered: value.ordered,
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::TupleAnswer> for domain::AnswerSpec {
    fn from(value: dto::TupleAnswer) -> Self {
        Self::Tuple {
            values: value.values.into_iter().map(Into::into).collect(),
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::MatrixAnswer> for domain::AnswerSpec {
    fn from(value: dto::MatrixAnswer) -> Self {
        Self::Matrix {
            rows: value
                .rows
                .into_iter()
                .map(|row| row.into_iter().map(Into::into).collect())
                .collect(),
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::ChoiceAnswer> for domain::AnswerSpec {
    fn from(value: dto::ChoiceAnswer) -> Self {
        Self::Choice {
            options: value.options.into_iter().map(Into::into).collect(),
            correct: value.correct,
            multiple: value.multiple,
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::MatchingAnswer> for domain::AnswerSpec {
    fn from(value: dto::MatchingAnswer) -> Self {
        Self::Matching {
            left: value.left.into_iter().map(Into::into).collect(),
            right: value.right.into_iter().map(Into::into).collect(),
            pairs: value.pairs,
            partial_credit: value.partial_credit,
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::SelfGradedAnswer> for domain::AnswerSpec {
    fn from(value: dto::SelfGradedAnswer) -> Self {
        Self::SelfGraded {
            rubric: value.rubric,
            feedback: value.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<dto::AnswerSpec> for domain::AnswerSpec {
    fn from(value: dto::AnswerSpec) -> Self {
        match value {
            dto::AnswerSpec::Numeric(value) => Self::Numeric(value.into()),
            dto::AnswerSpec::Expression(value) => Self::Expression(value.into()),
            dto::AnswerSpec::Equation(value) => Self::Equation(value.into()),
            dto::AnswerSpec::Set(value) => value.into(),
            dto::AnswerSpec::Tuple(value) => value.into(),
            dto::AnswerSpec::Matrix(value) => value.into(),
            dto::AnswerSpec::Choice(value) => value.into(),
            dto::AnswerSpec::Matching(value) => value.into(),
            dto::AnswerSpec::SelfGraded(value) => value.into(),
        }
    }
}

impl From<domain::AnswerSpec> for dto::AnswerSpec {
    fn from(value: domain::AnswerSpec) -> Self {
        match value {
            domain::AnswerSpec::Numeric(value) => Self::Numeric(value.into()),
            domain::AnswerSpec::Expression(value) => Self::Expression(value.into()),
            domain::AnswerSpec::Equation(value) => Self::Equation(value.into()),
            domain::AnswerSpec::Set {
                values,
                ordered,
                feedback,
            } => Self::Set(dto::SetAnswer {
                values: values.into_iter().map(Into::into).collect(),
                ordered,
                feedback: feedback.into_iter().map(Into::into).collect(),
            }),
            domain::AnswerSpec::Tuple { values, feedback } => Self::Tuple(dto::TupleAnswer {
                values: values.into_iter().map(Into::into).collect(),
                feedback: feedback.into_iter().map(Into::into).collect(),
            }),
            domain::AnswerSpec::Matrix { rows, feedback } => Self::Matrix(dto::MatrixAnswer {
                rows: rows
                    .into_iter()
                    .map(|row| row.into_iter().map(Into::into).collect())
                    .collect(),
                feedback: feedback.into_iter().map(Into::into).collect(),
            }),
            domain::AnswerSpec::Choice {
                options,
                correct,
                multiple,
                feedback,
            } => Self::Choice(dto::ChoiceAnswer {
                options: options.into_iter().map(Into::into).collect(),
                correct,
                multiple,
                feedback: feedback.into_iter().map(Into::into).collect(),
            }),
            domain::AnswerSpec::Matching {
                left,
                right,
                pairs,
                partial_credit,
                feedback,
            } => Self::Matching(dto::MatchingAnswer {
                left: left.into_iter().map(Into::into).collect(),
                right: right.into_iter().map(Into::into).collect(),
                pairs,
                partial_credit,
                feedback: feedback.into_iter().map(Into::into).collect(),
            }),
            domain::AnswerSpec::SelfGraded { rubric, feedback } => {
                Self::SelfGraded(dto::SelfGradedAnswer {
                    rubric,
                    feedback: feedback.into_iter().map(Into::into).collect(),
                })
            }
        }
    }
}
