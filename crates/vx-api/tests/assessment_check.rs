use serde_json::{json, Value};
use vx_api::{commands::ASSESSMENT_CHECK, dispatch};

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(request: Value) -> Result<Value, String> {
    dispatch(ASSESSMENT_CHECK, request).map_err(|error| format!("{error:?}"))
}

#[test]
fn authored_cases_match_the_json_contract_deterministically() -> TestResult {
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/assessment-cases.json"))?;
    let request_schema = schemars::schema_for!(vx_api::assessment_check::CheckRequest);
    let response_schema = schemars::schema_for!(vx_api::assessment_check::CheckResponse);
    let request_schema = serde_json::to_value(request_schema)?;
    let response_schema = serde_json::to_value(response_schema)?;
    let request_validator =
        jsonschema::JSONSchema::compile(&request_schema).map_err(|error| error.to_string())?;
    let response_validator =
        jsonschema::JSONSchema::compile(&response_schema).map_err(|error| error.to_string())?;
    for case in fixtures {
        let request = case["request"].clone();
        assert!(request_validator.is_valid(&request), "{}", case["name"]);
        let report = run(request.clone())?;
        assert!(response_validator.is_valid(&report), "{}", case["name"]);
        assert_eq!(report["status"], case["status"], "{}", case["name"]);
        assert_eq!(run(request)?, report);
    }
    Ok(())
}

fn numeric(response: &str) -> Value {
    json!({"exerciseId":"authored-test", "answer":{"type":"numeric", "value":"3"}, "response": response})
}

#[test]
fn syntax_has_position_and_does_not_become_incorrect() -> TestResult {
    let report = run(numeric("2 +"))?;
    assert_eq!(report["status"], "invalid_input");
    assert_eq!(report["inputError"]["field"], "response");
    assert_eq!(report["inputError"]["position"], 3);
    assert!(report["feedback"].is_null());
    assert!(report["sampling"].is_null());
    assert_eq!(run(numeric("sqrt(-1)"))?["status"], "incorrect");
    Ok(())
}

#[test]
fn expected_and_feedback_errors_are_author_errors_with_field_paths() -> TestResult {
    for (answer, field) in [
        (json!({"type":"numeric", "value":"2 +"}), "answer.value"),
        (json!({"type":"numeric", "value":"1/0"}), "answer.value"),
        (
            json!({"type":"numeric", "value":"3", "feedback":[{"matches":"x", "message":"Wrong"}]}),
            "answer.feedback[0].matches",
        ),
        (
            json!({"type":"expression", "expected":"z", "variables":["x"]}),
            "answer.expected",
        ),
        (
            json!({"type":"expression", "expected":"x", "variables":["x"], "samples":257}),
            "answer.samples",
        ),
    ] {
        let mut request = numeric("3");
        request["answer"] = answer;
        let error = dispatch(ASSESSMENT_CHECK, request)
            .err()
            .ok_or("Expected author error")?;
        assert_eq!(error.code, "invalid-request");
        assert_eq!(error.field.as_deref(), Some(field));
    }
    Ok(())
}

#[test]
fn correct_answer_takes_precedence_over_common_error_feedback() -> TestResult {
    let mut request = numeric("6/2");
    request["answer"]["feedback"] =
        json!([{"matches":"3", "message":"Do not show"}, {"matches":"4", "message":"Erro comum."}]);
    assert!(run(request.clone())?["feedback"].is_null());
    request["response"] = json!("8/2");
    assert_eq!(run(request)?["feedback"], "Erro comum.");
    Ok(())
}

#[test]
fn rejects_future_groups_malformed_payloads_and_expensive_inputs() -> TestResult {
    let mut request = numeric("3");
    request["answer"] = json!({"type":"code"});
    assert_eq!(
        dispatch(ASSESSMENT_CHECK, request)
            .err()
            .ok_or("Expected unsupported kind error")?
            .code,
        "bad-request"
    );
    let mut request = numeric("3");
    request["unexpected"] = json!(true);
    assert_eq!(
        dispatch(ASSESSMENT_CHECK, request)
            .err()
            .ok_or("Expected unknown field error")?
            .code,
        "bad-request"
    );
    for response in ["(".repeat(5000), "sin ".repeat(5000), "x^".repeat(5000)] {
        assert_eq!(run(numeric(&response))?["status"], "invalid_input");
    }
    Ok(())
}

#[test]
fn command_name_is_registered_in_front() {
    let front = include_str!("../../../apps/web/src/lib/adapters/engine/commands.ts");
    assert!(front.contains(&format!("assessmentCheck: '{ASSESSMENT_CHECK}'")));
}

#[test]
fn student_syntax_and_variable_failures_are_reports_not_author_errors() -> TestResult {
    let mut request = numeric("x² +");
    request["answer"] = json!({"type":"expression", "expected":"x^2", "variables":["x"]});
    let report = run(request.clone())?;
    assert_eq!(report["status"], "invalid_input");
    assert_eq!(report["inputError"]["position"], 4);
    request["response"] = json!("{x<0: x; z}");
    let report = run(request)?;
    assert_eq!(report["status"], "invalid_input");
    assert!(report["inputError"]["message"]
        .as_str()
        .is_some_and(|message| message.contains('z')));
    Ok(())
}

#[test]
fn configurable_tolerances_and_sampling_defaults_are_used() -> TestResult {
    let mut request = numeric("3.01");
    assert_eq!(run(request.clone())?["status"], "incorrect");
    request["answer"]["tolerance"] = json!({"abs":0.02, "rel":0});
    assert_eq!(run(request.clone())?["status"], "correct");
    request["answer"]["tolerance"] = json!({"abs":0, "rel":0});
    assert_eq!(run(request)?["status"], "incorrect");
    let report = run(
        json!({"exerciseId":"authored-test", "answer":{"type":"expression", "expected":"x+y", "variables":["x", "y"], "domain":{"x":[1,2]}, "samples":24}, "response":"y+x"}),
    )?;
    assert_eq!(report["sampling"]["requested"], 24);
    assert_eq!(report["sampling"]["valid"], 24);
    assert_eq!(report["sampling"]["domain"]["x"], json!([1.0, 2.0]));
    assert_eq!(report["sampling"]["domain"]["y"], json!([-10.0, 10.0]));
    Ok(())
}

#[test]
fn remaining_groups_preserve_field_paths_positions_and_manual_status() -> TestResult {
    let cases: Vec<Value> = serde_json::from_str(include_str!("fixtures/assessment-cases.json"))?;
    for (name, field, position) in [
        ("equation-syntax", "response", 4),
        ("tuple-invalid-component", "response.values[1]", 5),
    ] {
        let case = cases
            .iter()
            .find(|case| case["name"] == name)
            .ok_or("Missing fixture")?;
        let report = run(case["request"].clone())?;
        assert_eq!(report["inputError"]["field"], field);
        assert_eq!(report["inputError"]["position"], position);
    }
    for name in ["self-graded", "self-graded-empty"] {
        let case = cases
            .iter()
            .find(|case| case["name"] == name)
            .ok_or("Missing fixture")?;
        let report = run(case["request"].clone())?;
        assert_eq!(report["status"], "self_graded");
        assert!(report["feedback"].is_null());
        assert!(report["sampling"].is_null());
        assert!(report["inputError"].is_null());
    }
    Ok(())
}

#[test]
fn collection_and_objective_feedback_use_the_same_comparison_and_report_author_errors() -> TestResult
{
    let fixtures: Vec<Value> =
        serde_json::from_str(include_str!("fixtures/assessment-cases.json"))?;
    for (name, message) in [
        ("set-feedback", "Confira a segunda raiz."),
        ("choice-wrong", "Releia a definição."),
        ("equation-feedback", "Confira o sinal."),
    ] {
        let fixture = fixtures
            .iter()
            .find(|case| case["name"] == name)
            .ok_or("Missing fixture")?;
        assert_eq!(run(fixture["request"].clone())?["feedback"], message);
    }
    let mut request = numeric("(1)");
    request["answer"] = json!({"type":"tuple", "values":[{"type":"numeric", "value":"x"}]});
    let error = dispatch(ASSESSMENT_CHECK, request.clone())
        .err()
        .ok_or("Expected author error")?;
    assert_eq!(error.field.as_deref(), Some("answer.values[0].value"));
    request["answer"]["values"][0]["value"] = json!("1");
    request["answer"]["feedback"] = json!([{"matches":"(1;2)", "message":"Wrong shape"}]);
    let error = dispatch(ASSESSMENT_CHECK, request)
        .err()
        .ok_or("Expected feedback shape error")?;
    assert_eq!(error.field.as_deref(), Some("answer.feedback[0].matches"));
    Ok(())
}

#[test]
fn composite_inputs_and_objective_cardinalities_are_bounded_and_validated() -> TestResult {
    let mut request = numeric("{}");
    request["answer"] = json!({"type":"set", "ordered":false, "values":vec![json!({"type":"numeric","value":"1"});33]});
    let error = dispatch(ASSESSMENT_CHECK, request)
        .err()
        .ok_or("Expected collection limit")?;
    assert_eq!(error.field.as_deref(), Some("answer.values"));
    let mut request = numeric("[a;b]");
    request["answer"] = json!({"type":"choice", "options":[{"id":"a","text":"A"},{"id":"b","text":"B"}], "correct":["a"], "multiple":false});
    assert_eq!(run(request)?["status"], "invalid_input");
    let mut request = numeric("{a:1;a:1}");
    request["answer"] = json!({"type":"matching", "left":[{"id":"a","text":"A"}], "right":[{"id":"1","text":"1"}], "pairs":{"a":"1"}});
    assert_eq!(run(request)?["status"], "invalid_input");
    Ok(())
}
