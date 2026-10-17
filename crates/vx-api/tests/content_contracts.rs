use std::{error::Error, fs, path::Path};

use serde_json::{json, Value};
use vx_api::{assessment::AnswerSpec, content::*};
use vx_domain::shared::FieldError;

fn exercise_json() -> Result<Value, serde_json::Error> {
    serde_json::from_str(include_str!("fixtures/exercise.json"))
}

fn pack_json() -> Result<Value, serde_json::Error> {
    Ok(json!({
        "manifest": serde_json::from_str::<Value>(include_str!("fixtures/manifest.json"))?,
        "exercises": [exercise_json()?],
        "lists": [serde_json::from_str::<Value>(include_str!("fixtures/list.json"))?]
    }))
}

fn valid<T>(result: Result<T, Vec<FieldError>>) -> Result<T, Box<dyn Error>> {
    result.map_err(|errors| format!("{errors:?}").into())
}

fn assert_field<T: std::fmt::Debug>(result: Result<T, Vec<FieldError>>, field: &str) {
    assert!(result.is_err(), "expected validation failure");
    let errors = result.err().unwrap_or_default();
    assert!(
        errors.iter().any(|error| error.field == field),
        "{field}: {errors:?}"
    );
    assert!(errors.iter().all(|error| !error.message.is_empty()));
}

#[test]
fn decodes_fixtures_through_domain_validation_and_round_trips() -> Result<(), Box<dyn Error>> {
    let exercise = valid(decode_exercise(exercise_json()?))?;
    assert!(exercise.validate_for_publication().is_ok());
    let pack = valid(decode_pack(pack_json()?))?;
    assert!(pack.validate_for_publication().is_ok());
    let dto = valid(Pack::try_from(pack.clone()))?;
    assert_eq!(pack, valid(decode_pack(serde_json::to_value(dto)?))?);
    Ok(())
}

#[test]
fn reports_unsafe_reference_links_and_insufficient_metadata_by_field() -> Result<(), Box<dyn Error>>
{
    for url in [
        "javascript:alert(1)",
        "data:text/html,bad",
        "file:///private.pdf",
        "https:///missing-host",
    ] {
        let mut input = exercise_json()?;
        input["references"][0]["url"] = json!(url);
        assert_field(decode_exercise(input), "references[0].url");
    }
    for field in ["title", "license"] {
        let mut input = exercise_json()?;
        input["references"][0][field] = json!(" ");
        assert_field(decode_exercise(input), &format!("references[0].{field}"));
    }
    let mut input = exercise_json()?;
    input["references"][0]["url"] = Value::Null;
    assert_field(decode_exercise(input), "references[0].url");
    Ok(())
}

#[test]
fn reference_identifiers_are_alternatives_and_rights_do_not_leak_into_content(
) -> Result<(), Box<dyn Error>> {
    for (field, value) in [
        ("doi", "10.1000/xyz"),
        ("arxiv", "2401.01234v2"),
        ("isbn", "978-0-00-000000-0"),
    ] {
        let mut input = exercise_json()?;
        input["references"][0]["url"] = Value::Null;
        input["references"][0][field] = json!(value);
        input["references"][0]["license"] = json!("proprietary");
        assert!(valid(decode_exercise(input))?
            .validate_for_publication()
            .is_ok());
    }
    Ok(())
}

#[test]
fn rejects_private_or_restricted_content_at_publication_boundary() -> Result<(), Box<dyn Error>> {
    for license in ["proprietary", "unknown", "CC-BY-NC-SA-4.0", "CC-BY-ND-4.0"] {
        let mut input = exercise_json()?;
        input["source"]["license"] = json!(license);
        assert_field(decode_exercise(input), "source.license");
    }
    let mut input = exercise_json()?;
    input["visibility"] = json!("private");
    let exercise = valid(decode_exercise(input))?;
    assert_field(exercise.validate_for_publication(), "visibility");

    let mut input = pack_json()?;
    input["exercises"][0]["visibility"] = json!("private");
    assert_field(decode_pack(input), "exercises[0].visibility");
    Ok(())
}

#[test]
fn private_third_party_documents_stay_valid_only_locally() -> Result<(), Box<dyn Error>> {
    let mut input = exercise_json()?;
    input["visibility"] = json!("private");
    input["source"] = json!({"kind":"imported", "license":"proprietary", "citation":"Fixture sintética autoral de importação"});
    let exercise = valid(decode_exercise(input))?;
    assert!(exercise.validate().is_ok());
    assert_field(exercise.validate_for_publication(), "source.license");
    Ok(())
}

#[test]
fn pack_references_resolve_to_included_documents_and_parts() -> Result<(), Box<dyn Error>> {
    let mut input = pack_json()?;
    input["lists"][0]["items"][0]["parts"][0] = json!("missing");
    assert_field(decode_pack(input), "lists[0].items[0].parts[0]");
    let mut input = pack_json()?;
    input["manifest"]["lists"][0] = json!("0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e99");
    assert_field(decode_pack(input), "manifest.lists[0]");
    let mut input = pack_json()?;
    input["exercises"] = json!([]);
    assert_field(decode_pack(input), "lists[0].items[0].exercise");
    Ok(())
}

#[test]
fn uuid_identity_ignores_hexadecimal_letter_case() -> Result<(), Box<dyn Error>> {
    let mut input = pack_json()?;
    input["manifest"]["lists"][0] = json!(input["manifest"]["lists"][0]
        .as_str()
        .ok_or("list ID")?
        .to_uppercase());
    input["lists"][0]["items"][0]["exercise"] = json!(input["exercises"][0]["id"]
        .as_str()
        .ok_or("exercise ID")?
        .to_uppercase());
    assert!(decode_pack(input.clone()).is_ok());
    let mut duplicate = input["exercises"][0].clone();
    duplicate["id"] = json!(duplicate["id"]
        .as_str()
        .ok_or("exercise ID")?
        .to_uppercase());
    input["exercises"]
        .as_array_mut()
        .ok_or("exercise documents")?
        .push(duplicate);
    assert_field(decode_pack(input), "exercises[1]");

    let mut input = pack_json()?;
    let id = input["manifest"]["lists"][0]
        .as_str()
        .ok_or("list ID")?
        .to_uppercase();
    input["manifest"]["lists"]
        .as_array_mut()
        .ok_or("list IDs")?
        .push(json!(id));
    assert_field(decode_pack(input), "manifest.lists[1]");

    let mut input = pack_json()?;
    let mut item = input["lists"][0]["items"][0].clone();
    item["exercise"] = json!(item["exercise"]
        .as_str()
        .ok_or("exercise ID")?
        .to_uppercase());
    item["number"] = json!("2");
    input["lists"][0]["items"]
        .as_array_mut()
        .ok_or("list items")?
        .push(item);
    assert_field(decode_pack(input), "lists[0].items[1]");
    Ok(())
}

#[test]
fn tagged_answer_errors_keep_exact_leaf_and_element_paths() -> Result<(), Box<dyn Error>> {
    for (answer, field) in [
        (json!({"type":"numeric","value":42}), "value"),
        (
            json!({"type":"tuple","values":[{"type":"numeric"}]}),
            "values[0].value",
        ),
        (
            json!({"type":"matrix","rows":[[{"type":"numeric","value":false}]]}),
            "rows[0][0].value",
        ),
        (
            json!({"type":"tuple","values":[{"type":"expression","expected":"x","variables":[42]}]}),
            "values[0].variables[0]",
        ),
        (
            json!({"type":"tuple","values":[{"type":"equation","expected":"x=1","variables":["x"]}]}),
            "values[0].type",
        ),
    ] {
        let mut input = exercise_json()?;
        input["parts"][0]["answer"] = answer.clone();
        assert_field(decode_exercise(input), &format!("parts[0].answer.{field}"));
        let mut input = pack_json()?;
        input["exercises"][0]["parts"][0]["answer"] = answer;
        assert_field(
            decode_pack(input),
            &format!("exercises[0].parts[0].answer.{field}"),
        );
    }
    Ok(())
}

#[test]
fn decoding_preserves_structural_and_deserialization_field_errors() -> Result<(), Box<dyn Error>> {
    let mut input = exercise_json()?;
    input["revision"] = json!(0);
    input["parts"][0]["answer"] = json!({"type":"matrix", "rows":[[]]});
    let errors = decode_exercise(input).err().unwrap_or_default();
    assert!(errors.iter().any(|error| error.field == "revision"));
    assert!(errors
        .iter()
        .any(|error| error.field == "parts[0].answer.rows[0]"));

    let mut input = exercise_json()?;
    input["references"][0]["authors"] = json!(42);
    assert_field(decode_exercise(input), "references[0].authors");
    let mut input = exercise_json()?;
    input["references"][0]
        .as_object_mut()
        .ok_or("expected reference object")?
        .remove("license");
    assert_field(decode_exercise(input), "references[0].license");
    Ok(())
}

#[test]
fn minimal_authored_documents_round_trip() -> Result<(), Box<dyn Error>> {
    let pack: Pack = serde_json::from_value(pack_json()?)?;
    let serialized = serde_json::to_value(&pack)?;
    assert_eq!(pack, serde_json::from_value(serialized)?);
    assert_eq!(pack.exercises[0].revision, 1);
    assert_eq!(pack.lists[0].items[0].parts, Some(vec!["a".into()]));
    Ok(())
}

#[test]
fn every_f1_answer_kind_has_a_round_trip() -> Result<(), Box<dyn Error>> {
    let scalar = json!({"type":"numeric", "value":"1/2"});
    let answers = vec![
        scalar.clone(),
        json!({"type":"expression", "expected":"x^2", "variables":["x"]}),
        json!({"type":"equation", "expected":"y=x", "variables":["x","y"]}),
        json!({"type":"set", "ordered":false, "values":[]}),
        json!({"type":"tuple", "values":[scalar.clone()]}),
        json!({"type":"matrix", "rows":[[scalar]]}),
        json!({"type":"choice", "options":[{"id":"a","text":"Um"}], "correct":["a"], "multiple":false}),
        json!({"type":"matching", "left":[{"id":"a","text":"Um"}], "right":[{"id":"b","text":"1"}], "pairs":{"a":"b"}}),
        json!({"type":"self_graded", "rubric":"Compare com a solução."}),
    ];
    for value in answers {
        let answer: AnswerSpec = serde_json::from_value(value.clone())?;
        let mut input = exercise_json()?;
        input["parts"][0]["answer"] = value;
        let exercise = valid(decode_exercise(input))?;
        let normalized = valid(Exercise::try_from(exercise))?;
        assert_eq!(answer, normalized.parts[0].answer);
        assert_eq!(
            answer,
            serde_json::from_value(serde_json::to_value(&answer)?)?
        );
    }
    Ok(())
}

#[test]
fn technical_payloads_round_trip_and_reject_nonfinite_export() -> Result<(), Box<dyn Error>> {
    let mut input = exercise_json()?;
    let params = json!({"f":"x^2", "x0":1.5, "labels":[true, null, "a"], "nested":{"x":-2}, "maximum":u64::MAX});
    input["parts"][0]["viz"] = json!({"id":"calculus.derivative@1", "params":params});
    input["parts"][0]["solution"] = json!({"engine":{"command":"calculus.sample", "request":{"expression":"x^2", "start":-1, "end":1, "sampleCount":12}}});
    let mut exercise = valid(decode_exercise(input))?;
    let dto = valid(Exercise::try_from(exercise.clone()))?;
    assert_eq!(
        serde_json::to_value(&dto)?["parts"][0]["viz"]["params"],
        params
    );
    let engine = serde_json::to_value(&dto)?["parts"][0]["solution"]["engine"].clone();
    assert!(vx_api::dispatch("calculus.sample", engine["request"].clone()).is_ok());
    assert_eq!(
        exercise,
        valid(decode_exercise(serde_json::to_value(dto)?))?
    );
    let viz = exercise.parts[0].viz.as_mut().ok_or("visualization")?;
    viz.params = vx_domain::shared::JsonValue::Number(f64::NAN);
    assert_field(Exercise::try_from(exercise), "parts[0].viz.params");
    Ok(())
}

#[test]
fn rejects_unsupported_and_forged_fields_instead_of_discarding_them() -> Result<(), Box<dyn Error>>
{
    for field in ["verify", "evidence"] {
        let mut input = exercise_json()?;
        input["parts"][0][field] = json!([]);
        assert!(
            serde_json::from_value::<Exercise>(input).is_err(),
            "{field}"
        );
    }
    for (field, value) in [("params", json!({"seed":1})), ("material", json!("id"))] {
        let mut input = exercise_json()?;
        if field == "material" {
            input["references"][0][field] = value;
        } else {
            input[field] = value;
        }
        assert!(
            serde_json::from_value::<Exercise>(input).is_err(),
            "{field}"
        );
    }
    let mut input = exercise_json()?;
    input["references"][0]["file"] = json!("document.pdf");
    assert!(serde_json::from_value::<Exercise>(input).is_err());
    for kind in ["code", "sql"] {
        assert!(serde_json::from_value::<AnswerSpec>(json!({"type":kind})).is_err());
    }
    Ok(())
}

#[test]
fn collections_reject_strings_and_recursive_specifications() {
    for element in [json!("1"), json!({"type":"tuple", "values":[]})] {
        let answer = json!({"type":"tuple", "values":[element]});
        assert!(serde_json::from_value::<AnswerSpec>(answer).is_err());
    }
}

#[test]
fn generated_schemas_accept_fixtures_and_reject_wrong_contract_shapes() -> Result<(), Box<dyn Error>>
{
    for (name, schema) in vx_api::contracts::schemas() {
        let value = serde_json::to_value(schema)?;
        let validator =
            jsonschema::JSONSchema::compile(&value).map_err(|error| error.to_string())?;
        let valid = match name {
            "exercise" => exercise_json()?,
            "exercise-list" => pack_json()?["lists"][0].clone(),
            "pack-manifest" => pack_json()?["manifest"].clone(),
            "pack" => pack_json()?,
            "answer-spec" => json!({"type":"numeric", "value":"5"}),
            "assessment-check-request" => {
                json!({"exerciseId":"authored-test", "answer":{"type":"numeric", "value":"5"}, "response":"5"})
            }
            "assessment-check-response" => {
                json!({"status":"correct", "sampling":null, "feedback":null, "inputError":null})
            }
            _ => unreachable!(),
        };
        assert!(validator.is_valid(&valid), "{name}");
        assert!(!validator.is_valid(&json!({"unexpected":true})), "{name}");
        if name == "exercise" {
            for url in [
                "javascript:alert(1)",
                "data:text/html,bad",
                "file:///private.pdf",
            ] {
                let mut invalid = valid.clone();
                invalid["references"][0]["url"] = json!(url);
                assert!(!validator.is_valid(&invalid), "{url}");
            }
        }
        if name == "answer-spec" {
            for invalid in [
                json!({"type":"tuple", "values":["1"]}),
                json!({"type":"numeric", "value":"1", "evidence":[]}),
                json!({"type":"code", "language":"rust"}),
                json!({"type":"set", "values":[], "ordered":true}),
                json!({"type":"matrix", "rows":[[]]}),
            ] {
                assert!(!validator.is_valid(&invalid), "{invalid}");
            }
        }
    }
    Ok(())
}

#[test]
fn generated_contracts_are_current_and_deterministic() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let typescript = vx_api::contracts::typescript();
    assert!(typescript.contains("\"type\": \"numeric\""));
    assert!(typescript.contains("\"type\": \"self_graded\""));
    assert!(!typescript.contains(": any"));
    assert_eq!(typescript, vx_api::contracts::typescript());
    assert_eq!(
        typescript,
        fs::read_to_string(root.join("apps/web/src/lib/domain/generated/content.ts"))?
    );
    for (name, schema) in vx_api::contracts::schemas() {
        let content = format!("{}\n", serde_json::to_string_pretty(&schema)?);
        assert_eq!(
            content,
            fs::read_to_string(root.join(format!("contracts/{name}.schema.json")))?
        );
    }
    Ok(())
}

#[test]
fn inner_layers_do_not_gain_serialization_dependencies() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for layer in ["vx-domain", "vx-application", "vx-infra"] {
        let manifest = fs::read_to_string(root.join(layer).join("Cargo.toml"))?;
        for forbidden in ["serde", "schemars", "ts-rs"] {
            assert!(!manifest.contains(forbidden), "{layer}: {forbidden}");
        }
    }
    Ok(())
}
