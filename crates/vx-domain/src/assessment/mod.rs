//! Public answer specifications and pure grading rules for all F1 answer kinds.

mod collections;
mod comparison;
mod forms;
mod spec;
mod validation;

pub use collections::{combine_verdicts, compare_choices, compare_pairs, match_set};
pub use comparison::{
    compare_equations, compare_expressions, compare_numbers, Comparison, SamplingReport,
    SamplingSettings, Verdict,
};
pub use forms::check_form;

pub use spec::{
    AnswerOption, AnswerSpec, EquationAnswerSpec, ExpressionAnswerSpec, ExpressionForm, Feedback,
    NumericAnswerSpec, SamplingDomain, ScalarAnswerSpec, Tolerance,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod comparison_tests;

#[cfg(test)]
mod grading_tests;
