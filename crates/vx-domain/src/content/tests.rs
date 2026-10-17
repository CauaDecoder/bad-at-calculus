use std::collections::BTreeMap;

use super::*;
use crate::assessment::{AnswerSpec, NumericAnswerSpec};
use crate::shared::{FieldError, JsonValue, License, SkillId, Visibility};

const EXERCISE_ID: &str = "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e90";
const LIST_ID: &str = "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e91";
const PACK_ID: &str = "0192f7a4-6c1e-7b3a-9d2e-5f8a1c3b7e92";

fn source() -> Source {
    Source {
        kind: SourceKind::Authored,
        citation: None,
        license: License::CcBySa40,
        authors: vec!["Vértice".into()],
    }
}

fn part() -> Part {
    Part {
        label: "a".into(),
        prompt: "Calcule $1/2$.".into(),
        answer: AnswerSpec::Numeric(NumericAnswerSpec {
            value: "1/2".into(),
            tolerance: None,
            unit: None,
            feedback: vec![],
        }),
        hints: vec![],
        solution: None,
        viz: None,
        points: None,
    }
}

fn exercise() -> Exercise {
    Exercise {
        schema_version: 1,
        id: EXERCISE_ID.into(),
        revision: 1,
        locale: "pt-BR".into(),
        title: "Fração".into(),
        statement: "Considere a fração.".into(),
        parts: vec![part()],
        skills: vec![SkillId("matematica.fracoes".into())],
        difficulty: None,
        estimated_minutes: None,
        assets: vec![],
        source: source(),
        references: vec![],
        visibility: Visibility::Public,
    }
}

fn list() -> ExerciseList {
    ExerciseList {
        schema_version: 1,
        id: LIST_ID.into(),
        title: "Frações".into(),
        description: None,
        items: vec![ListItem {
            exercise: EXERCISE_ID.into(),
            number: "1".into(),
            parts: Some(vec!["a".into()]),
        }],
        source: source(),
        visibility: Visibility::Public,
    }
}

fn pack() -> Pack {
    Pack {
        manifest: PackManifest {
            schema_version: 1,
            id: PACK_ID.into(),
            name: "Frações".into(),
            description: None,
            license: License::CcBySa40,
            visibility: Visibility::Public,
            lists: vec![LIST_ID.into()],
        },
        exercises: vec![exercise()],
        lists: vec![list()],
    }
}

fn reference() -> Reference {
    Reference {
        kind: ReferenceType::Book,
        title: "Uma edição de cálculo".into(),
        authors: vec![],
        url: Some("https://example.org/book".into()),
        doi: None,
        arxiv: None,
        isbn: None,
        locator: None,
        license: License::from("CC-BY-NC-SA-4.0"),
    }
}

fn has_field(result: Result<(), Vec<FieldError>>, field: &str) {
    let errors = result.err().unwrap_or_default();
    assert!(
        errors.iter().any(|error| error.field == field),
        "Missing {field}: {errors:?}"
    );
    assert!(errors
        .iter()
        .all(|error| !error.field.starts_with('.') && !error.message.is_empty()));
}

#[test]
fn minimal_public_exercise_list_and_pack_validate() {
    assert_eq!(exercise().validate(), Ok(()));
    assert_eq!(exercise().validate_for_publication(), Ok(()));
    assert_eq!(list().validate_for_publication(), Ok(()));
    assert_eq!(pack().validate_for_publication(), Ok(()));
    let mut standalone = pack();
    standalone.manifest.lists.clear();
    standalone.lists.clear();
    assert!(standalone.validate().is_ok());
}

#[test]
fn private_documents_validate_locally_but_never_publish() {
    let mut exercise = exercise();
    exercise.visibility = Visibility::Private;
    exercise.source.kind = SourceKind::Imported;
    exercise.source.citation = Some("Fonte privada de teste, sem conteúdo copiado".into());
    exercise.source.license = License::Proprietary;
    assert!(exercise.validate().is_ok());
    has_field(exercise.validate_for_publication(), "visibility");
    has_field(exercise.validate_for_publication(), "source.license");
    exercise.visibility = Visibility::Public;
    has_field(exercise.validate(), "source.license");
    let mut list = list();
    list.visibility = Visibility::Private;
    assert!(list.validate().is_ok());
    has_field(list.validate_for_publication(), "visibility");
}

#[test]
fn public_pack_cannot_promote_private_documents_even_when_unlisted() {
    let mut pack = pack();
    pack.exercises[0].visibility = Visibility::Private;
    pack.lists[0].visibility = Visibility::Private;
    pack.manifest.lists.clear();
    has_field(pack.validate(), "exercises[0].visibility");
    has_field(pack.validate_for_publication(), "lists[0].visibility");
    pack.manifest.visibility = Visibility::Private;
    assert!(pack.validate().is_ok());
    has_field(pack.validate_for_publication(), "manifest.visibility");
    has_field(pack.validate_for_publication(), "exercises[0].visibility");
}

#[test]
fn authored_public_content_requires_cc_by_sa_and_authors() {
    let mut source = source();
    for license in [
        License::CcBy40,
        License::Cc0,
        License::Proprietary,
        License::Unknown,
    ] {
        source.license = license;
        has_field(source.validate_for_publication(), "license");
    }
    source.license = License::CcBySa40;
    source.authors.clear();
    has_field(source.validate(), "authors");
}

#[test]
fn imported_and_adapted_open_content_require_attribution_and_compatible_rights() {
    for kind in [SourceKind::Imported, SourceKind::Adapted] {
        let mut source = source();
        source.kind = kind;
        has_field(source.validate(), "citation");
        source.citation = Some("Fonte aberta de teste".into());
        for license in [License::CcBySa40, License::CcBy40, License::Cc0] {
            source.license = license;
            assert!(source.validate_for_publication().is_ok());
        }
        for license in [
            "CC-BY-NC-SA-4.0",
            "CC-BY-ND-4.0",
            "CC-BY-NC-4.0",
            "CC-BY-NC-ND-4.0",
            "unknown",
            "proprietary",
            "MIT",
        ] {
            source.license = License::from(license);
            has_field(source.validate_for_publication(), "license");
        }
    }
}

#[test]
fn references_allow_restrictive_editions_as_links_without_relicensing_exercises() {
    let mut exercise = exercise();
    exercise.references.push(reference());
    assert!(exercise.validate_for_publication().is_ok());
    exercise.references[0].license = License::Proprietary;
    assert!(exercise.validate_for_publication().is_ok());
    exercise.references[0].url = Some("javascript:alert(1)".into());
    has_field(exercise.validate(), "references[0].url");
}

#[test]
fn reference_requires_identification_and_nonblank_metadata() {
    let mut reference = reference();
    reference.url = None;
    has_field(reference.validate(), "url");
    for identifier in ["doi", "arxiv", "isbn"] {
        let mut identified = reference.clone();
        match identifier {
            "doi" => identified.doi = Some("10.1000/test".into()),
            "arxiv" => identified.arxiv = Some("2401.01234".into()),
            _ => identified.isbn = Some("9780000000000".into()),
        }
        assert!(identified.validate().is_ok());
    }
    reference.title.clear();
    reference.doi = Some(" ".into());
    reference.license = License::Other("".into());
    has_field(reference.validate(), "title");
    has_field(reference.validate(), "doi");
    has_field(reference.validate(), "license");
}

#[test]
fn exercise_reports_identity_revision_and_metadata_fields() {
    let mut exercise = exercise();
    exercise.schema_version = 2;
    exercise.id = "bad".into();
    exercise.revision = 0;
    exercise.locale.clear();
    exercise.title.clear();
    exercise.statement.clear();
    exercise.parts.clear();
    exercise.skills.clear();
    exercise.difficulty = Some(6);
    exercise.estimated_minutes = Some(0);
    for field in [
        "schemaVersion",
        "id",
        "revision",
        "locale",
        "title",
        "statement",
        "parts",
        "skills",
        "difficulty",
        "estimatedMinutes",
    ] {
        has_field(exercise.validate(), field);
    }
}

#[test]
fn exercise_rejects_duplicate_parts_skills_and_assets() {
    let mut exercise = exercise();
    exercise.parts.push(part());
    exercise.skills.push(exercise.skills[0].clone());
    let asset = Asset {
        id: "graph".into(),
        path: "assets/graph.svg".into(),
        alt: "Gráfico autoral".into(),
    };
    exercise.assets = vec![asset.clone(), asset];
    for field in ["parts[1]", "skills[1]", "assets[1]"] {
        has_field(exercise.validate(), field);
    }
    exercise.skills[0] = SkillId("Bad..skill".into());
    has_field(exercise.validate(), "skills[0]");
}

#[test]
fn pack_rejects_duplicate_documents_and_manifest_ids() {
    let mut pack = pack();
    pack.exercises.push(exercise());
    pack.lists.push(list());
    pack.manifest.lists.push(LIST_ID.into());
    for field in ["exercises[1]", "lists[1]", "manifest.lists[1]"] {
        has_field(pack.validate(), field);
    }
}

#[test]
fn pack_resolves_manifest_lists_exercises_and_selected_parts() {
    let mut pack = pack();
    pack.manifest.lists = vec![PACK_ID.into()];
    pack.lists[0].items[0].parts = Some(vec!["z".into()]);
    has_field(pack.validate(), "manifest.lists[0]");
    has_field(pack.validate(), "lists[0].items[0].parts[0]");
    pack.lists[0].items[0].exercise = PACK_ID.into();
    has_field(pack.validate(), "lists[0].items[0].exercise");
}

#[test]
fn manifest_requires_identity_name_schema_and_compatible_public_license() {
    let mut manifest = pack().manifest;
    manifest.id.clear();
    manifest.name.clear();
    manifest.schema_version = 0;
    manifest.license = License::Unknown;
    manifest.lists = vec!["lists/a.json".into()];
    for field in ["id", "name", "schemaVersion", "license", "lists[0]"] {
        has_field(manifest.validate(), field);
    }
}

#[test]
fn list_items_require_valid_ids_numbers_and_distinct_part_selections() {
    let mut list = list();
    list.items[0].exercise = "not-a-uuid".into();
    list.items[0].number.clear();
    list.items[0].parts = Some(vec!["a".into(), "a".into(), " ".into()]);
    for field in [
        "items[0].exercise",
        "items[0].number",
        "items[0].parts[1]",
        "items[0].parts[2]",
    ] {
        has_field(list.validate(), field);
    }
    list.items[0].parts = Some(vec![]);
    has_field(list.validate(), "items[0].parts");
    list.items.push(list.items[0].clone());
    has_field(list.validate(), "items[1]");
}

#[test]
fn parts_propagate_answer_and_editorial_errors() {
    let mut part = part();
    part.label.clear();
    part.prompt.clear();
    part.hints = vec!["".into()];
    part.points = Some(f64::NAN);
    part.answer = AnswerSpec::Numeric(NumericAnswerSpec {
        value: "".into(),
        tolerance: None,
        unit: None,
        feedback: vec![],
    });
    part.solution = Some(Solution {
        steps: vec![],
        engine: None,
    });
    part.viz = Some(Visualization {
        id: "".into(),
        params: JsonValue::Number(f64::INFINITY),
    });
    for field in [
        "label",
        "prompt",
        "hints[0]",
        "points",
        "answer.value",
        "solution.steps",
        "viz.id",
        "viz.params",
    ] {
        has_field(part.validate(), field);
    }
}

#[test]
fn technical_payloads_remain_pure_json_and_reject_nonfinite_numbers() {
    let payload = JsonValue::Object(BTreeMap::from([
        ("expression".into(), JsonValue::String("x^2".into())),
        (
            "details".into(),
            JsonValue::Array(vec![
                JsonValue::Null,
                JsonValue::Bool(true),
                JsonValue::Number(2.0),
            ]),
        ),
    ]));
    let mut call = EngineCall {
        command: "calculus.differentiate".into(),
        request: payload,
    };
    assert!(call.validate().is_ok());
    assert!(Solution {
        steps: vec![],
        engine: Some(call.clone())
    }
    .validate()
    .is_ok());
    call.request = JsonValue::Object(BTreeMap::from([(
        "x".into(),
        JsonValue::Array(vec![JsonValue::Number(f64::NAN)]),
    )]));
    has_field(call.validate(), "request.x[0]");
}

#[test]
fn assets_reject_traversal_absolute_paths_and_missing_accessibility_text() {
    for path in [
        "../x.svg",
        "/assets/a.svg",
        "assets/../a.svg",
        "assets/./a.svg",
        "assets//a.svg",
        "assets\\a.svg",
        "C:/assets/a.svg",
        "assets/a\0.svg",
    ] {
        has_field(
            Asset {
                id: "a".into(),
                path: path.into(),
                alt: "Gráfico".into(),
            }
            .validate(),
            "path",
        );
    }
    has_field(
        Asset {
            id: "".into(),
            path: "assets/a.svg".into(),
            alt: "".into(),
        }
        .validate(),
        "alt",
    );
}
