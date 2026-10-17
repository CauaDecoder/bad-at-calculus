use super::*;
use crate::use_cases::test_support::NumberParser;
use crate::use_cases::test_support::{FailingParser, FakeParserReturningXSquared};
use vx_domain::assessment::{ExpressionAnswerSpec, NumericAnswerSpec};

fn request(answer: AnswerSpec) -> CheckAnswerRequest {
    CheckAnswerRequest {
        exercise_id: "authored-fixture".into(),
        answer,
        response: "x^2".into(),
    }
}

fn expression() -> AnswerSpec {
    AnswerSpec::Expression(ExpressionAnswerSpec {
        expected: "x^2".into(),
        variables: vec!["x".into()],
        domain: None,
        form: None,
        samples: None,
        tolerance: None,
        feedback: vec![],
    })
}

#[test]
fn uses_parser_port_and_returns_a_sampled_report() -> Result<(), ApplicationError> {
    let report = CheckAnswer::new(FakeParserReturningXSquared).execute(&request(expression()))?;
    assert_eq!(
        report
            .comparison
            .as_ref()
            .map(|comparison| comparison.verdict),
        Some(Verdict::Correct)
    );
    assert_eq!(
        report
            .comparison
            .and_then(|comparison| comparison.sampling)
            .map(|sampling| sampling.valid),
        Some(12)
    );
    assert!(report.input_issue.is_none());
    Ok(())
}

#[test]
fn authored_parse_failure_is_not_a_student_attempt() {
    let error = CheckAnswer::new(FailingParser)
        .execute(&request(expression()))
        .err();
    assert!(
        matches!(error, Some(ApplicationError::InvalidField { field, .. }) if field == "answer.expected")
    );
}

#[test]
fn undeclared_variable_is_rejected_even_in_an_unreachable_branch() {
    let tree = Expr::Piecewise(vec![vx_domain::PiecewiseBranch {
        condition: None,
        value: Expr::variable("z"),
    }]);
    assert_eq!(unknown_variable(&tree, &["x".into()]), Some("z"));
}

#[test]
fn self_graded_is_never_marked_automatically_correct() -> Result<(), ApplicationError> {
    let check = CheckAnswer::new(FakeParserReturningXSquared);
    let report = check.execute(&request(AnswerSpec::SelfGraded {
        rubric: None,
        feedback: vec![],
    }))?;
    assert_eq!(
        report.comparison.map(|comparison| comparison.verdict),
        Some(Verdict::SelfGraded)
    );
    let mut spec = expression();
    if let AnswerSpec::Expression(spec) = &mut spec {
        spec.form = Some(vx_domain::assessment::ExpressionForm::Factored);
    }
    let report = check.execute(&request(spec))?;
    assert_eq!(
        report.comparison.map(|comparison| comparison.verdict),
        Some(Verdict::Correct)
    );
    Ok(())
}

#[test]
fn numeric_expected_must_be_a_constant() {
    let spec = AnswerSpec::Numeric(NumericAnswerSpec {
        value: "x".into(),
        tolerance: None,
        unit: None,
        feedback: vec![],
    });
    assert!(CheckAnswer::new(FakeParserReturningXSquared)
        .execute(&request(spec))
        .is_err());
}

#[test]
fn source_guard_bounds_recursion_and_rejects_definition_prefixes() {
    for source in [
        "(".repeat(200),
        "sin ".repeat(100),
        "x^".repeat(100),
        " ".repeat(4097),
        "y = 2".into(),
    ] {
        assert!(guard_source(&source).is_err());
    }
    assert!(guard_source("{x <= 0: -x; x}").is_ok());
}

#[test]
fn seed_is_stable_and_uuid_case_insensitive() {
    assert_eq!(
        seed_for("0192F7A4-6C1E-7B3A-9D2E-5F8A1C3B7E90"),
        seed_for("0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90")
    );
    assert_ne!(seed_for("fixture-1"), seed_for("fixture-2"));
    assert_eq!(seed_for("hello"), 1_335_831_723);
}

#[test]
fn ordered_and_unordered_collections_use_the_same_scalar_port() -> Result<(), ApplicationError> {
    let values = ["1", "2"]
        .map(|value| {
            ScalarAnswerSpec::Numeric(NumericAnswerSpec {
                value: value.into(),
                tolerance: None,
                unit: None,
                feedback: vec![],
            })
        })
        .to_vec();
    for (answer, response, expected) in [
        (
            AnswerSpec::Set {
                values: values.clone(),
                ordered: false,
                feedback: vec![],
            },
            "{2;1}",
            Verdict::Correct,
        ),
        (
            AnswerSpec::Tuple {
                values,
                feedback: vec![],
            },
            "(2;1)",
            Verdict::Incorrect,
        ),
    ] {
        let mut request = request(answer);
        request.response = response.into();
        let report = CheckAnswer::new(NumberParser).execute(&request)?;
        assert_eq!(
            report.comparison.map(|comparison| comparison.verdict),
            Some(expected)
        );
    }
    Ok(())
}

#[test]
fn collection_variable_error_identifies_the_component() -> Result<(), ApplicationError> {
    let mut request = request(AnswerSpec::Tuple {
        values: vec![ScalarAnswerSpec::Numeric(NumericAnswerSpec {
            value: "1".into(),
            tolerance: None,
            unit: None,
            feedback: vec![],
        })],
        feedback: vec![],
    });
    request.response = "(z)".into();
    let report = CheckAnswer::new(NumberParser).execute(&request)?;
    assert!(report.comparison.is_none());
    assert_eq!(
        report.input_issue.map(|issue| issue.field),
        Some("response.values[0]".into())
    );
    Ok(())
}

#[test]
fn collection_source_parser_preserves_decimal_commas_and_piecewise_separators() -> Result<(), String>
{
    let parts =
        input::delimited("(0,5; {x<0: -x; x})", ('(', ')')).map_err(|issue| issue.message)?;
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].source.trim(), "0,5");
    assert_eq!(parts[1].source.trim(), "{x<0: -x; x}");
    assert!(input::delimited("(1;[2)", ('(', ')')).is_err());
    Ok(())
}

#[test]
fn quoted_objective_ids_preserve_delimiters_quotes_and_escapes() -> Result<(), String> {
    let ids = input::identifiers(r#"["a;b";"a\"b";"c\\d"]"#).map_err(|issue| issue.message)?;
    assert_eq!(ids, ["a;b", "a\"b", "c\\d"]);
    for source in [r#"["a\q"]"#, r#"["a]"#, "[a;;b]", "[a;a]"] {
        assert!(input::identifiers(source).is_err(), "{source}");
    }
    Ok(())
}
