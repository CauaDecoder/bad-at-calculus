use std::collections::BTreeMap;

/// Optional absolute and relative numerical comparison thresholds.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tolerance {
    /// Absolute threshold; absence leaves the grader's default unchanged.
    pub abs: Option<f64>,
    /// Relative threshold; absence leaves the grader's default unchanged.
    pub rel: Option<f64>,
}

/// Sampling intervals keyed by declared variable names.
pub type SamplingDomain = BTreeMap<String, [f64; 2]>;

/// A requested expression form, checked by the grader rather than string comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpressionForm {
    /// A product of factors.
    Factored,
    /// An expanded sum.
    Expanded,
    /// An expression without negative exponents.
    NoNegativeExponents,
}

/// A common wrong answer and the message to show when it matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    /// Expected wrong answer, interpreted with the answer's grading algorithm.
    pub matches: String,
    /// Markdown feedback for the student.
    pub message: String,
}

/// A constant mathematical expression, optionally carrying a unit.
#[derive(Debug, Clone, PartialEq)]
pub struct NumericAnswerSpec {
    /// Constant expression such as `1/2` or `sqrt(3)`.
    pub value: String,
    /// Optional comparison thresholds.
    pub tolerance: Option<Tolerance>,
    /// Optional unit label.
    pub unit: Option<String>,
    /// Common wrong answers.
    pub feedback: Vec<Feedback>,
}

/// An expression and the explicit variables used to compare it.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionAnswerSpec {
    /// Expected mathematical expression.
    pub expected: String,
    /// Distinct variable identifiers.
    pub variables: Vec<String>,
    /// Optional per-variable sampling intervals.
    pub domain: Option<SamplingDomain>,
    /// Optional structural form requirement.
    pub form: Option<ExpressionForm>,
    /// Optional number of sampling points.
    pub samples: Option<u32>,
    /// Optional comparison thresholds.
    pub tolerance: Option<Tolerance>,
    /// Common wrong answers.
    pub feedback: Vec<Feedback>,
}

/// An equation compared by proportional residuals in F1.2.
#[derive(Debug, Clone, PartialEq)]
pub struct EquationAnswerSpec {
    /// Expected equation with one equality sign and two nonblank sides.
    pub expected: String,
    /// Distinct variable identifiers.
    pub variables: Vec<String>,
    /// Optional per-variable sampling intervals.
    pub domain: Option<SamplingDomain>,
    /// Optional number of sampling points.
    pub samples: Option<u32>,
    /// Optional comparison thresholds.
    pub tolerance: Option<Tolerance>,
    /// Common wrong answers.
    pub feedback: Vec<Feedback>,
}

/// Nonrecursive collection elements, each with its own comparison settings.
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarAnswerSpec {
    /// Constant numerical answer.
    Numeric(NumericAnswerSpec),
    /// Variable-dependent expression.
    Expression(ExpressionAnswerSpec),
}

/// A stable identifier and display text for a choice or matching side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerOption {
    /// Identifier used in correct choices or matching pairs.
    pub id: String,
    /// Markdown display text.
    pub text: String,
}

/// The nine supported F1 answer kinds; no grading or evidence is implied by validity.
#[derive(Debug, Clone, PartialEq)]
pub enum AnswerSpec {
    /// A constant mathematical expression.
    Numeric(NumericAnswerSpec),
    /// An algebraic expression.
    Expression(ExpressionAnswerSpec),
    /// An equation with declared variables.
    Equation(EquationAnswerSpec),
    /// An unordered collection, including the empty set.
    Set {
        /// Expected scalar elements.
        values: Vec<ScalarAnswerSpec>,
        /// Must be false; ordered collections use `Tuple`.
        ordered: bool,
        /// Common wrong answers.
        feedback: Vec<Feedback>,
    },
    /// A nonempty ordered collection.
    Tuple {
        /// Expected scalar elements in order.
        values: Vec<ScalarAnswerSpec>,
        /// Common wrong answers.
        feedback: Vec<Feedback>,
    },
    /// A nonempty rectangular matrix.
    Matrix {
        /// Rows of scalar elements.
        rows: Vec<Vec<ScalarAnswerSpec>>,
        /// Common wrong answers.
        feedback: Vec<Feedback>,
    },
    /// Single or multiple choices, compared as a set of identifiers.
    Choice {
        /// Available options with distinct identifiers.
        options: Vec<AnswerOption>,
        /// Distinct correct option identifiers.
        correct: Vec<String>,
        /// Allows more than one correct option.
        multiple: bool,
        /// Common wrong answers.
        feedback: Vec<Feedback>,
    },
    /// A mapping from every left option to an existing right option.
    Matching {
        /// Left-side options with distinct identifiers.
        left: Vec<AnswerOption>,
        /// Right-side options with distinct identifiers.
        right: Vec<AnswerOption>,
        /// Correct right identifier for each left identifier.
        pairs: BTreeMap<String, String>,
        /// Whether grading may award credit per pair.
        partial_credit: bool,
        /// Common wrong answers.
        feedback: Vec<Feedback>,
    },
    /// Manual comparison with a rubric, never automatic correctness.
    SelfGraded {
        /// Optional guidance for self-assessment.
        rubric: Option<String>,
        /// Common wrong answers.
        feedback: Vec<Feedback>,
    },
}
