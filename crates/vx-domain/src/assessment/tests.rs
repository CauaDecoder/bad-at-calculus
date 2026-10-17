use std::collections::BTreeMap;

use super::*;
use crate::shared::FieldError;

fn scalar() -> ScalarAnswerSpec {
    ScalarAnswerSpec::Numeric(numeric())
}

fn numeric() -> NumericAnswerSpec {
    NumericAnswerSpec {
        value: "1/2".into(),
        tolerance: None,
        unit: None,
        feedback: vec![],
    }
}

fn expression() -> ExpressionAnswerSpec {
    ExpressionAnswerSpec {
        expected: "x^2".into(),
        variables: vec!["x".into()],
        domain: None,
        form: None,
        samples: None,
        tolerance: None,
        feedback: vec![],
    }
}

fn option(id: &str) -> AnswerOption {
    AnswerOption {
        id: id.into(),
        text: format!("Option {id}"),
    }
}

fn has_field(result: Result<(), Vec<FieldError>>, field: &str) {
    let errors = result.err().unwrap_or_default();
    assert!(
        errors.iter().any(|error| error.field == field),
        "Missing {field}: {errors:?}"
    );
    assert!(errors.iter().all(|error| !error.message.is_empty()));
}

#[test]
fn all_nine_answer_kinds_have_minimal_valid_specs() {
    let answers = vec![
        AnswerSpec::Numeric(numeric()),
        AnswerSpec::Expression(expression()),
        AnswerSpec::Equation(EquationAnswerSpec {
            expected: "y = 2*x".into(),
            variables: vec!["x".into(), "y".into()],
            domain: None,
            samples: None,
            tolerance: None,
            feedback: vec![],
        }),
        AnswerSpec::Set {
            values: vec![],
            ordered: false,
            feedback: vec![],
        },
        AnswerSpec::Tuple {
            values: vec![scalar(), ScalarAnswerSpec::Expression(expression())],
            feedback: vec![],
        },
        AnswerSpec::Matrix {
            rows: vec![vec![scalar()]],
            feedback: vec![],
        },
        AnswerSpec::Choice {
            options: vec![option("a"), option("b")],
            correct: vec!["a".into()],
            multiple: false,
            feedback: vec![],
        },
        AnswerSpec::Matching {
            left: vec![option("a")],
            right: vec![option("1")],
            pairs: BTreeMap::from([("a".into(), "1".into())]),
            partial_credit: false,
            feedback: vec![],
        },
        AnswerSpec::SelfGraded {
            rubric: None,
            feedback: vec![],
        },
    ];
    assert_eq!(answers.len(), 9);
    for answer in answers {
        assert_eq!(answer.validate(), Ok(()), "{answer:?}");
    }
}

#[test]
fn tolerance_rejects_nonfinite_and_negative_values_but_allows_exact_zero() {
    for value in [-1.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
        has_field(
            Tolerance {
                abs: Some(value),
                rel: None,
            }
            .validate(),
            "abs",
        );
        has_field(
            Tolerance {
                abs: None,
                rel: Some(value),
            }
            .validate(),
            "rel",
        );
    }
    assert!(Tolerance {
        abs: Some(0.0),
        rel: Some(0.0)
    }
    .validate()
    .is_ok());
}

#[test]
fn numeric_errors_keep_field_paths_without_parsing_constants() {
    let mut spec = numeric();
    spec.value = " ".into();
    spec.unit = Some(" ".into());
    spec.feedback.push(Feedback {
        matches: "".into(),
        message: "".into(),
    });
    has_field(spec.validate(), "value");
    has_field(spec.validate(), "unit");
    has_field(spec.validate(), "feedback[0].matches");
    has_field(spec.validate(), "feedback[0].message");
    spec.value = "sqrt(3)/2".into();
}

#[test]
fn expression_requires_distinct_identifiers_and_valid_sampling_bounds() {
    let mut spec = expression();
    spec.variables = vec!["x".into(), "x".into(), "2y".into()];
    spec.domain = Some(BTreeMap::from([
        ("z".into(), [0.0, 1.0]),
        ("x".into(), [1.0, 1.0]),
    ]));
    spec.samples = Some(0);
    has_field(spec.validate(), "variables[1]");
    has_field(spec.validate(), "variables[2]");
    has_field(spec.validate(), "domain.z");
    has_field(spec.validate(), "domain.x");
    has_field(spec.validate(), "samples");
    spec.variables.clear();
    has_field(spec.validate(), "variables");
}

#[test]
fn sampling_rejects_infinity_nan_and_reversed_intervals() {
    for bounds in [[f64::NAN, 1.0], [0.0, f64::INFINITY], [2.0, -2.0]] {
        let mut spec = expression();
        spec.domain = Some(BTreeMap::from([("x".into(), bounds)]));
        has_field(spec.validate(), "domain.x");
    }
    let mut spec = expression();
    spec.domain = Some(BTreeMap::from([("x".into(), [-1.0, 1.0])]));
    spec.samples = Some(12);
    spec.form = Some(ExpressionForm::Factored);
    assert!(spec.validate().is_ok());
}

#[test]
fn sampling_requires_at_least_twelve_points() {
    for samples in [0, 1, 11] {
        let mut spec = expression();
        spec.samples = Some(samples);
        has_field(spec.validate(), "samples");
    }
    for samples in [12, 24] {
        let mut spec = expression();
        spec.samples = Some(samples);
        assert!(spec.validate().is_ok());
    }
}

#[test]
fn equations_require_exactly_two_nonblank_sides() {
    for expected in ["", "x", "= 1", "x =", "x = y = 1", "x == 1"] {
        has_field(
            EquationAnswerSpec {
                expected: expected.into(),
                variables: vec!["x".into()],
                domain: None,
                samples: None,
                tolerance: None,
                feedback: vec![],
            }
            .validate(),
            "expected",
        );
    }
}

#[test]
fn collection_invariants_and_nested_element_paths_are_preserved() {
    has_field(
        AnswerSpec::Set {
            values: vec![],
            ordered: true,
            feedback: vec![],
        }
        .validate(),
        "ordered",
    );
    has_field(
        AnswerSpec::Tuple {
            values: vec![],
            feedback: vec![],
        }
        .validate(),
        "values",
    );
    has_field(
        AnswerSpec::Matrix {
            rows: vec![],
            feedback: vec![],
        }
        .validate(),
        "rows",
    );
    has_field(
        AnswerSpec::Matrix {
            rows: vec![vec![]],
            feedback: vec![],
        }
        .validate(),
        "rows[0]",
    );
    has_field(
        AnswerSpec::Matrix {
            rows: vec![vec![scalar()], vec![scalar(), scalar()]],
            feedback: vec![],
        }
        .validate(),
        "rows[1]",
    );
    let mut bad = numeric();
    bad.value.clear();
    let matrix = AnswerSpec::Matrix {
        rows: vec![vec![ScalarAnswerSpec::Numeric(bad)]],
        feedback: vec![],
    };
    has_field(matrix.validate(), "rows[0][0].value");
}

#[test]
fn choices_reject_duplicate_and_missing_ids_and_single_choice_ambiguity() {
    let answer = AnswerSpec::Choice {
        options: vec![option("a"), option("a")],
        correct: vec!["a".into(), "a".into(), "z".into()],
        multiple: false,
        feedback: vec![],
    };
    has_field(answer.validate(), "options[1]");
    has_field(answer.validate(), "correct");
    has_field(answer.validate(), "correct[1]");
    has_field(answer.validate(), "correct[2]");
    has_field(
        AnswerSpec::Choice {
            options: vec![],
            correct: vec![],
            multiple: true,
            feedback: vec![],
        }
        .validate(),
        "options",
    );
    has_field(
        AnswerSpec::Choice {
            options: vec![option("a")],
            correct: vec![],
            multiple: true,
            feedback: vec![],
        }
        .validate(),
        "correct",
    );
}

#[test]
fn matching_requires_coverage_and_existing_endpoints() {
    let answer = AnswerSpec::Matching {
        left: vec![option("a"), option("b")],
        right: vec![option("1")],
        pairs: BTreeMap::from([("a".into(), "9".into()), ("z".into(), "1".into())]),
        partial_credit: true,
        feedback: vec![],
    };
    has_field(answer.validate(), "pairs.b");
    has_field(answer.validate(), "pairs.a");
    has_field(answer.validate(), "pairs.z");
}

#[test]
fn self_graded_rubric_is_optional_but_not_blank() {
    has_field(
        AnswerSpec::SelfGraded {
            rubric: Some(" ".into()),
            feedback: vec![],
        }
        .validate(),
        "rubric",
    );
}
