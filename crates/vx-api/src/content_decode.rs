//! JSON boundary decoding with canonical field paths for transport failures.

use serde::de::DeserializeOwned;
use serde_json::Value;
use vx_domain::{content as domain, shared::FieldError};

use crate::assessment as answer;
use crate::content as dto;

/// Decodes an exercise and validates its canonical domain invariants.
pub fn decode_exercise(value: Value) -> Result<domain::Exercise, Vec<FieldError>> {
    check_exercise_answers(&value, "")?;
    decode::<dto::Exercise>(&value)?.try_into()
}

/// Decodes a list and validates its local metadata and entries.
pub fn decode_list(value: Value) -> Result<domain::ExerciseList, Vec<FieldError>> {
    decode::<dto::ExerciseList>(&value)?.try_into()
}

/// Decodes and validates a pack manifest, independently of archive transport.
pub fn decode_manifest(value: Value) -> Result<domain::PackManifest, Vec<FieldError>> {
    decode::<dto::PackManifest>(&value)?.try_into()
}

/// Decodes a complete pack and validates documents and cross-document references.
pub fn decode_pack(value: Value) -> Result<domain::Pack, Vec<FieldError>> {
    if let Some(exercises) = value.get("exercises").and_then(Value::as_array) {
        for (index, exercise) in exercises.iter().enumerate() {
            check_exercise_answers(exercise, &format!("exercises[{index}]."))?;
        }
    }
    decode::<dto::Pack>(&value)?.try_into()
}

// Serde buffers internally tagged enum payloads, erasing their inner paths.
// Decode each selected payload as its derived struct first, including scalar
// collection elements. This preserves location without guessing from error text.
fn check_exercise_answers(value: &Value, prefix: &str) -> Result<(), Vec<FieldError>> {
    if let Some(parts) = value.get("parts").and_then(Value::as_array) {
        for (index, part) in parts.iter().enumerate() {
            if let Some(answer) = part.get("answer") {
                check_answer(answer, &format!("{prefix}parts[{index}].answer"))?;
            }
        }
    }
    Ok(())
}

fn answer_fields<'a>(value: &'a Value, prefix: &str) -> Result<(&'a str, Value), Vec<FieldError>> {
    let object = value.as_object().ok_or_else(|| {
        vec![FieldError {
            field: prefix.into(),
            message: "Expected an answer specification object".into(),
        }]
    })?;
    let kind = object.get("type").and_then(Value::as_str).ok_or_else(|| {
        vec![FieldError {
            field: format!("{prefix}.type"),
            message: "Expected an answer type string".into(),
        }]
    })?;
    let mut fields = object.clone();
    fields.remove("type");
    Ok((kind, Value::Object(fields)))
}

fn check_payload<T: DeserializeOwned>(value: &Value, prefix: &str) -> Result<(), Vec<FieldError>> {
    decode::<T>(value).map(|_| ()).map_err(|errors| {
        errors
            .into_iter()
            .map(|mut error| {
                error.field = if error.field.is_empty() {
                    prefix.into()
                } else {
                    format!("{prefix}.{}", error.field)
                };
                error
            })
            .collect()
    })
}

fn unsupported_answer(kind: &str, prefix: &str) -> Vec<FieldError> {
    vec![FieldError {
        field: format!("{prefix}.type"),
        message: format!("Unsupported answer type `{kind}`"),
    }]
}

fn check_scalar(value: &Value, prefix: &str) -> Result<(), Vec<FieldError>> {
    let (kind, fields) = answer_fields(value, prefix)?;
    match kind {
        "numeric" => check_payload::<answer::NumericAnswer>(&fields, prefix),
        "expression" => check_payload::<answer::ExpressionAnswer>(&fields, prefix),
        _ => Err(unsupported_answer(kind, prefix)),
    }
}

fn check_values(value: Option<&Value>, prefix: &str) -> Result<(), Vec<FieldError>> {
    if let Some(values) = value.and_then(Value::as_array) {
        for (index, value) in values.iter().enumerate() {
            check_scalar(value, &format!("{prefix}[{index}]"))?;
        }
    }
    Ok(())
}

fn check_answer(value: &Value, prefix: &str) -> Result<(), Vec<FieldError>> {
    let (kind, fields) = answer_fields(value, prefix)?;
    match kind {
        "numeric" => check_payload::<answer::NumericAnswer>(&fields, prefix),
        "expression" => check_payload::<answer::ExpressionAnswer>(&fields, prefix),
        "equation" => check_payload::<answer::EquationAnswer>(&fields, prefix),
        "set" => {
            check_values(fields.get("values"), &format!("{prefix}.values"))?;
            check_payload::<answer::SetAnswer>(&fields, prefix)
        }
        "tuple" => {
            check_values(fields.get("values"), &format!("{prefix}.values"))?;
            check_payload::<answer::TupleAnswer>(&fields, prefix)
        }
        "matrix" => {
            if let Some(rows) = fields.get("rows").and_then(Value::as_array) {
                for (index, row) in rows.iter().enumerate() {
                    check_values(Some(row), &format!("{prefix}.rows[{index}]"))?;
                }
            }
            check_payload::<answer::MatrixAnswer>(&fields, prefix)
        }
        "choice" => check_payload::<answer::ChoiceAnswer>(&fields, prefix),
        "matching" => check_payload::<answer::MatchingAnswer>(&fields, prefix),
        "self_graded" => check_payload::<answer::SelfGradedAnswer>(&fields, prefix),
        _ => Err(unsupported_answer(kind, prefix)),
    }
}

fn decode<T: DeserializeOwned>(value: &Value) -> Result<T, Vec<FieldError>> {
    serde_path_to_error::deserialize(value).map_err(|error| {
        let message = error.inner().to_string();
        let mut field = error.path().to_string();
        if field == "." {
            field.clear();
        }
        // Missing and unknown fields point at their parent in serde's path.
        if let Some(name) = named_field(&message) {
            if field != name && !field.ends_with(&format!(".{name}")) {
                if !field.is_empty() {
                    field.push('.');
                }
                field.push_str(name);
            }
        }
        vec![FieldError { field, message }]
    })
}

fn named_field(message: &str) -> Option<&str> {
    ["missing field `", "unknown field `", "duplicate field `"]
        .into_iter()
        .find_map(|prefix| message.strip_prefix(prefix)?.split('`').next())
}
