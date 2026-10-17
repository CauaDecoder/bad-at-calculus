use crate::shared::{FieldError, Validation};

use super::{
    AnswerOption, AnswerSpec, EquationAnswerSpec, ExpressionAnswerSpec, Feedback,
    NumericAnswerSpec, SamplingDomain, ScalarAnswerSpec, Tolerance,
};

impl Tolerance {
    /// Rejects nonfinite and negative thresholds without interpreting expressions.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        for (field, value) in [("abs", self.abs), ("rel", self.rel)] {
            if let Some(value) = value {
                validation.require(
                    value.is_finite() && value >= 0.0,
                    field,
                    "Must be finite and nonnegative",
                );
            }
        }
        validation.finish()
    }
}

impl Feedback {
    /// Requires a wrong answer and a nonblank explanatory message.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.matches, "matches");
        validation.text(&self.message, "message");
        validation.finish()
    }
}

impl NumericAnswerSpec {
    /// Checks structural fields only; expression parsing belongs to F1.2.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.value, "value");
        validation.optional_text(&self.unit, "unit");
        validate_tolerance(&self.tolerance, &mut validation);
        validate_feedback(&self.feedback, &mut validation);
        validation.finish()
    }
}

impl ExpressionAnswerSpec {
    /// Checks variables, intervals, samples, tolerance and editorial feedback.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.expected, "expected");
        validate_variables(&self.variables, &mut validation);
        validate_sampling(&self.domain, &self.variables, self.samples, &mut validation);
        validate_tolerance(&self.tolerance, &mut validation);
        validate_feedback(&self.feedback, &mut validation);
        validation.finish()
    }
}

impl EquationAnswerSpec {
    /// Checks the equation's two sides and sampling structure, not mathematical syntax.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        let sides: Vec<_> = self.expected.split('=').collect();
        validation.require(
            sides.len() == 2 && sides.iter().all(|side| !side.trim().is_empty()),
            "expected",
            "Must contain exactly one equality sign with two nonblank sides",
        );
        validate_variables(&self.variables, &mut validation);
        validate_sampling(&self.domain, &self.variables, self.samples, &mut validation);
        validate_tolerance(&self.tolerance, &mut validation);
        validate_feedback(&self.feedback, &mut validation);
        validation.finish()
    }
}

impl ScalarAnswerSpec {
    /// Validates this element independently of its containing collection.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        match self {
            Self::Numeric(spec) => spec.validate(),
            Self::Expression(spec) => spec.validate(),
        }
    }
}

impl AnswerOption {
    /// Requires a stable identifier and nonblank display text.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        validation.text(&self.id, "id");
        validation.text(&self.text, "text");
        validation.finish()
    }
}

impl AnswerSpec {
    /// Validates the F1 structure without grading, parsing or producing evidence.
    pub fn validate(&self) -> Result<(), Vec<FieldError>> {
        let mut validation = Validation::default();
        match self {
            Self::Numeric(spec) => return spec.validate(),
            Self::Expression(spec) => return spec.validate(),
            Self::Equation(spec) => return spec.validate(),
            Self::Set {
                values,
                ordered,
                feedback,
            } => {
                validation.require(!ordered, "ordered", "A set must be unordered");
                validate_values(values, "values", &mut validation);
                validate_feedback(feedback, &mut validation);
            }
            Self::Tuple { values, feedback } => {
                validation.require(!values.is_empty(), "values", "A tuple must not be empty");
                validate_values(values, "values", &mut validation);
                validate_feedback(feedback, &mut validation);
            }
            Self::Matrix { rows, feedback } => {
                validate_rows(rows, &mut validation);
                validate_feedback(feedback, &mut validation);
            }
            Self::Choice {
                options,
                correct,
                multiple,
                feedback,
            } => {
                validate_choice(options, correct, *multiple, &mut validation);
                validate_feedback(feedback, &mut validation);
            }
            Self::Matching {
                left,
                right,
                pairs,
                feedback,
                ..
            } => {
                validate_matching(left, right, pairs, &mut validation);
                validate_feedback(feedback, &mut validation);
            }
            Self::SelfGraded { rubric, feedback } => {
                validation.optional_text(rubric, "rubric");
                validate_feedback(feedback, &mut validation);
            }
        }
        validation.finish()
    }
}

fn validate_tolerance(tolerance: &Option<Tolerance>, validation: &mut Validation) {
    if let Some(tolerance) = tolerance {
        validation.nested(tolerance.validate(), "tolerance");
    }
}

fn validate_feedback(feedback: &[Feedback], validation: &mut Validation) {
    for (index, item) in feedback.iter().enumerate() {
        validation.nested(item.validate(), &format!("feedback[{index}]"));
    }
}

fn validate_variables(variables: &[String], validation: &mut Validation) {
    validation.require(
        !variables.is_empty(),
        "variables",
        "At least one variable is required",
    );
    validation.unique(variables.iter().map(String::as_str), "variables");
    for (index, variable) in variables.iter().enumerate() {
        let mut characters = variable.chars();
        let valid = characters
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
            && characters.all(|character| character.is_ascii_alphanumeric() || character == '_');
        validation.require(
            valid,
            &format!("variables[{index}]"),
            "Must be a variable identifier",
        );
    }
}

fn validate_sampling(
    domain: &Option<SamplingDomain>,
    variables: &[String],
    samples: Option<u32>,
    validation: &mut Validation,
) {
    if let Some(samples) = samples {
        validation.require(samples >= 12, "samples", "At least 12 samples are required");
    }
    if let Some(domain) = domain {
        for (variable, [lower, upper]) in domain {
            let field = format!("domain.{variable}");
            validation.require(
                variables.contains(variable),
                &field,
                "Variable is not declared",
            );
            validation.require(
                lower.is_finite() && upper.is_finite() && lower < upper,
                &field,
                "Sampling interval must have finite increasing bounds",
            );
        }
    }
}

fn validate_values(values: &[ScalarAnswerSpec], field: &str, validation: &mut Validation) {
    for (index, value) in values.iter().enumerate() {
        validation.nested(value.validate(), &format!("{field}[{index}]"));
    }
}

fn validate_rows(rows: &[Vec<ScalarAnswerSpec>], validation: &mut Validation) {
    validation.require(
        !rows.is_empty(),
        "rows",
        "A matrix must have at least one row",
    );
    let width = rows.first().map_or(0, Vec::len);
    for (index, row) in rows.iter().enumerate() {
        let field = format!("rows[{index}]");
        validation.require(
            !row.is_empty() && row.len() == width,
            &field,
            "Rows must have the same positive number of columns",
        );
        validate_values(row, &field, validation);
    }
}

fn validate_options(options: &[AnswerOption], field: &str, validation: &mut Validation) {
    validation.require(
        !options.is_empty(),
        field,
        "At least one option is required",
    );
    validation.unique(options.iter().map(|option| option.id.as_str()), field);
    for (index, option) in options.iter().enumerate() {
        validation.nested(option.validate(), &format!("{field}[{index}]"));
    }
}

fn validate_choice(
    options: &[AnswerOption],
    correct: &[String],
    multiple: bool,
    validation: &mut Validation,
) {
    validate_options(options, "options", validation);
    validation.require(
        !correct.is_empty(),
        "correct",
        "At least one correct option is required",
    );
    validation.require(
        multiple || correct.len() == 1,
        "correct",
        "Single choice requires exactly one correct option",
    );
    validation.unique(correct.iter().map(String::as_str), "correct");
    for (index, id) in correct.iter().enumerate() {
        validation.require(
            options.iter().any(|option| option.id == *id),
            &format!("correct[{index}]"),
            "Correct option does not exist",
        );
    }
}

fn validate_matching(
    left: &[AnswerOption],
    right: &[AnswerOption],
    pairs: &std::collections::BTreeMap<String, String>,
    validation: &mut Validation,
) {
    validate_options(left, "left", validation);
    validate_options(right, "right", validation);
    for option in left {
        validation.require(
            pairs.contains_key(&option.id),
            &format!("pairs.{}", option.id),
            "Missing left-side match",
        );
    }
    for (left_id, right_id) in pairs {
        let field = format!("pairs.{left_id}");
        validation.require(
            left.iter().any(|option| option.id == *left_id),
            &field,
            "Left option does not exist",
        );
        validation.require(
            right.iter().any(|option| option.id == *right_id),
            &field,
            "Right option does not exist",
        );
    }
}
