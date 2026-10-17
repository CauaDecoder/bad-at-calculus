//! Canonical DTO/domain conversion without serialization dependencies in the domain.

use serde_json::Value;
use vx_domain::{content as domain, shared};

use crate::content as dto;

impl From<dto::Visibility> for shared::Visibility {
    fn from(value: dto::Visibility) -> Self {
        match value {
            dto::Visibility::Public => Self::Public,
            dto::Visibility::Private => Self::Private,
        }
    }
}

impl From<shared::Visibility> for dto::Visibility {
    fn from(value: shared::Visibility) -> Self {
        match value {
            shared::Visibility::Public => Self::Public,
            shared::Visibility::Private => Self::Private,
        }
    }
}

impl From<dto::SourceKind> for domain::SourceKind {
    fn from(value: dto::SourceKind) -> Self {
        match value {
            dto::SourceKind::Authored => Self::Authored,
            dto::SourceKind::Imported => Self::Imported,
            dto::SourceKind::Adapted => Self::Adapted,
        }
    }
}

impl From<domain::SourceKind> for dto::SourceKind {
    fn from(value: domain::SourceKind) -> Self {
        match value {
            domain::SourceKind::Authored => Self::Authored,
            domain::SourceKind::Imported => Self::Imported,
            domain::SourceKind::Adapted => Self::Adapted,
        }
    }
}

impl From<dto::Source> for domain::Source {
    fn from(value: dto::Source) -> Self {
        Self {
            kind: value.kind.into(),
            citation: value.citation,
            license: value.license.as_str().into(),
            authors: value.authors,
        }
    }
}

impl From<domain::Source> for dto::Source {
    fn from(value: domain::Source) -> Self {
        Self {
            kind: value.kind.into(),
            citation: value.citation,
            license: value.license.as_str().into(),
            authors: value.authors,
        }
    }
}

impl From<dto::ReferenceType> for domain::ReferenceType {
    fn from(value: dto::ReferenceType) -> Self {
        match value {
            dto::ReferenceType::Paper => Self::Paper,
            dto::ReferenceType::Book => Self::Book,
            dto::ReferenceType::Notes => Self::Notes,
            dto::ReferenceType::Video => Self::Video,
            dto::ReferenceType::Webpage => Self::Webpage,
        }
    }
}

impl From<domain::ReferenceType> for dto::ReferenceType {
    fn from(value: domain::ReferenceType) -> Self {
        match value {
            domain::ReferenceType::Paper => Self::Paper,
            domain::ReferenceType::Book => Self::Book,
            domain::ReferenceType::Notes => Self::Notes,
            domain::ReferenceType::Video => Self::Video,
            domain::ReferenceType::Webpage => Self::Webpage,
        }
    }
}

impl From<dto::Reference> for domain::Reference {
    fn from(value: dto::Reference) -> Self {
        Self {
            kind: value.kind.into(),
            title: value.title,
            authors: value.authors,
            url: value.url,
            doi: value.doi,
            arxiv: value.arxiv,
            isbn: value.isbn,
            locator: value.locator,
            license: value.license.as_str().into(),
        }
    }
}

impl From<domain::Reference> for dto::Reference {
    fn from(value: domain::Reference) -> Self {
        Self {
            kind: value.kind.into(),
            title: value.title,
            authors: value.authors,
            url: value.url,
            doi: value.doi,
            arxiv: value.arxiv,
            isbn: value.isbn,
            locator: value.locator,
            license: value.license.as_str().into(),
            material: None,
        }
    }
}

impl From<dto::Asset> for domain::Asset {
    fn from(value: dto::Asset) -> Self {
        Self {
            id: value.id,
            path: value.path,
            alt: value.alt,
        }
    }
}

impl From<domain::Asset> for dto::Asset {
    fn from(value: domain::Asset) -> Self {
        Self {
            id: value.id,
            path: value.path,
            alt: value.alt,
        }
    }
}

fn import_json(value: Value, path: &str) -> Result<shared::JsonValue, Vec<shared::FieldError>> {
    use shared::JsonValue;
    Ok(match value {
        Value::Null => JsonValue::Null,
        Value::Bool(value) => JsonValue::Bool(value),
        Value::Number(value) => import_number(value, path)?,
        Value::String(value) => JsonValue::String(value),
        Value::Array(values) => JsonValue::Array(
            values
                .into_iter()
                .enumerate()
                .map(|(index, value)| import_json(value, &format!("{path}[{index}]")))
                .collect::<Result<_, _>>()?,
        ),
        Value::Object(values) => JsonValue::Object(
            values
                .into_iter()
                .map(|(key, value)| {
                    import_json(value, &join_path(path, &key)).map(|value| (key, value))
                })
                .collect::<Result<_, _>>()?,
        ),
    })
}

fn import_number(
    value: serde_json::Number,
    path: &str,
) -> Result<shared::JsonValue, Vec<shared::FieldError>> {
    if let Some(integer) = value.as_i64() {
        return Ok(shared::JsonValue::Integer(integer));
    }
    if let Some(integer) = value.as_u64() {
        return Ok(shared::JsonValue::Unsigned(integer));
    }
    value
        .as_f64()
        .filter(|value| value.is_finite())
        .map(shared::JsonValue::Number)
        .ok_or_else(|| {
            vec![shared::FieldError {
                field: path.into(),
                message: "JSON number cannot be represented as a finite f64".into(),
            }]
        })
}

fn export_json(value: shared::JsonValue, path: &str) -> Result<Value, Vec<shared::FieldError>> {
    use shared::JsonValue;
    Ok(match value {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(value) => Value::Bool(value),
        JsonValue::Integer(value) => Value::Number(value.into()),
        JsonValue::Unsigned(value) => Value::Number(value.into()),
        JsonValue::Number(value) => {
            Value::Number(serde_json::Number::from_f64(value).ok_or_else(|| {
                vec![shared::FieldError {
                    field: path.into(),
                    message: "JSON numbers must be finite".into(),
                }]
            })?)
        }
        JsonValue::String(value) => Value::String(value),
        JsonValue::Array(values) => Value::Array(
            values
                .into_iter()
                .enumerate()
                .map(|(index, value)| export_json(value, &format!("{path}[{index}]")))
                .collect::<Result<_, _>>()?,
        ),
        JsonValue::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(key, value)| {
                    export_json(value, &join_path(path, &key)).map(|value| (key, value))
                })
                .collect::<Result<_, _>>()?,
        ),
    })
}

fn join_path(prefix: &str, field: &str) -> String {
    if prefix.is_empty() {
        field.into()
    } else if field.is_empty() {
        prefix.into()
    } else if field.starts_with('[') {
        format!("{prefix}{field}")
    } else {
        format!("{prefix}.{field}")
    }
}

fn at<T>(
    result: Result<T, Vec<shared::FieldError>>,
    prefix: &str,
) -> Result<T, Vec<shared::FieldError>> {
    result.map_err(|errors| {
        errors
            .into_iter()
            .map(|mut error| {
                error.field = join_path(prefix, &error.field);
                error
            })
            .collect()
    })
}

impl TryFrom<dto::EngineRequest> for domain::EngineCall {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::EngineRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            command: value.command,
            request: import_json(value.request, "request")?,
        })
    }
}

impl TryFrom<domain::EngineCall> for dto::EngineRequest {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: domain::EngineCall) -> Result<Self, Self::Error> {
        Ok(Self {
            command: value.command,
            request: export_json(value.request, "request")?,
        })
    }
}

impl TryFrom<dto::Solution> for domain::Solution {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::Solution) -> Result<Self, Self::Error> {
        Ok(Self {
            steps: value.steps,
            engine: at(value.engine.map(TryInto::try_into).transpose(), "engine")?,
        })
    }
}

impl TryFrom<domain::Solution> for dto::Solution {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: domain::Solution) -> Result<Self, Self::Error> {
        Ok(Self {
            steps: value.steps,
            engine: at(value.engine.map(TryInto::try_into).transpose(), "engine")?,
        })
    }
}

impl TryFrom<dto::Visualization> for domain::Visualization {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::Visualization) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            params: import_json(value.params, "params")?,
        })
    }
}

impl TryFrom<domain::Visualization> for dto::Visualization {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: domain::Visualization) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            params: export_json(value.params, "params")?,
        })
    }
}

impl TryFrom<dto::Part> for domain::Part {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::Part) -> Result<Self, Self::Error> {
        Ok(Self {
            label: value.label,
            prompt: value.prompt,
            answer: value.answer.into(),
            hints: value.hints,
            solution: at(
                value.solution.map(TryInto::try_into).transpose(),
                "solution",
            )?,
            viz: at(value.viz.map(TryInto::try_into).transpose(), "viz")?,
            points: value.points,
        })
    }
}

impl TryFrom<domain::Part> for dto::Part {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: domain::Part) -> Result<Self, Self::Error> {
        Ok(Self {
            label: value.label,
            prompt: value.prompt,
            answer: value.answer.into(),
            hints: value.hints,
            solution: at(
                value.solution.map(TryInto::try_into).transpose(),
                "solution",
            )?,
            viz: at(value.viz.map(TryInto::try_into).transpose(), "viz")?,
            points: value.points,
        })
    }
}

fn import_exercise(value: dto::Exercise) -> Result<domain::Exercise, Vec<shared::FieldError>> {
    Ok(domain::Exercise {
        schema_version: value.schema_version,
        id: value.id,
        revision: value.revision,
        locale: value.locale,
        title: value.title,
        statement: value.statement,
        parts: value
            .parts
            .into_iter()
            .enumerate()
            .map(|(index, part)| at(part.try_into(), &format!("parts[{index}]")))
            .collect::<Result<_, _>>()?,
        skills: value.skills.into_iter().map(shared::SkillId).collect(),
        difficulty: value.difficulty,
        estimated_minutes: value.estimated_minutes,
        assets: value.assets.into_iter().map(Into::into).collect(),
        source: value.source.into(),
        references: value.references.into_iter().map(Into::into).collect(),
        visibility: value.visibility.into(),
    })
}

impl TryFrom<dto::Exercise> for domain::Exercise {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::Exercise) -> Result<Self, Self::Error> {
        let exercise = import_exercise(value)?;
        exercise.validate()?;
        Ok(exercise)
    }
}

impl TryFrom<domain::Exercise> for dto::Exercise {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: domain::Exercise) -> Result<Self, Self::Error> {
        value.validate()?;
        Ok(Self {
            schema_version: value.schema_version,
            id: value.id,
            revision: value.revision,
            locale: value.locale,
            title: value.title,
            statement: value.statement,
            parts: value
                .parts
                .into_iter()
                .enumerate()
                .map(|(index, part)| at(part.try_into(), &format!("parts[{index}]")))
                .collect::<Result<_, _>>()?,
            skills: value.skills.into_iter().map(|skill| skill.0).collect(),
            difficulty: value.difficulty,
            estimated_minutes: value.estimated_minutes,
            assets: value.assets.into_iter().map(Into::into).collect(),
            source: value.source.into(),
            references: value.references.into_iter().map(Into::into).collect(),
            visibility: value.visibility.into(),
            params: None,
        })
    }
}

impl From<dto::ListItem> for domain::ListItem {
    fn from(value: dto::ListItem) -> Self {
        Self {
            exercise: value.exercise,
            number: value.number,
            parts: value.parts,
        }
    }
}

impl From<domain::ListItem> for dto::ListItem {
    fn from(value: domain::ListItem) -> Self {
        Self {
            exercise: value.exercise,
            number: value.number,
            parts: value.parts,
        }
    }
}

fn import_list(value: dto::ExerciseList) -> domain::ExerciseList {
    domain::ExerciseList {
        schema_version: value.schema_version,
        id: value.id,
        title: value.title,
        description: value.description,
        items: value.items.into_iter().map(Into::into).collect(),
        source: value.source.into(),
        visibility: value.visibility.into(),
    }
}

impl TryFrom<dto::ExerciseList> for domain::ExerciseList {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::ExerciseList) -> Result<Self, Self::Error> {
        let list = import_list(value);
        list.validate()?;
        Ok(list)
    }
}

impl From<domain::ExerciseList> for dto::ExerciseList {
    fn from(value: domain::ExerciseList) -> Self {
        Self {
            schema_version: value.schema_version,
            id: value.id,
            title: value.title,
            description: value.description,
            items: value.items.into_iter().map(Into::into).collect(),
            source: value.source.into(),
            visibility: value.visibility.into(),
        }
    }
}

fn import_manifest(value: dto::PackManifest) -> domain::PackManifest {
    domain::PackManifest {
        schema_version: value.schema_version,
        id: value.id,
        name: value.name,
        description: value.description,
        license: value.license.as_str().into(),
        visibility: value.visibility.into(),
        lists: value.lists,
    }
}

impl TryFrom<dto::PackManifest> for domain::PackManifest {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::PackManifest) -> Result<Self, Self::Error> {
        let manifest = import_manifest(value);
        manifest.validate()?;
        Ok(manifest)
    }
}

impl From<domain::PackManifest> for dto::PackManifest {
    fn from(value: domain::PackManifest) -> Self {
        Self {
            schema_version: value.schema_version,
            id: value.id,
            name: value.name,
            description: value.description,
            license: value.license.as_str().into(),
            visibility: value.visibility.into(),
            lists: value.lists,
        }
    }
}

impl TryFrom<dto::Pack> for domain::Pack {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: dto::Pack) -> Result<Self, Self::Error> {
        let pack = Self {
            manifest: import_manifest(value.manifest),
            exercises: value
                .exercises
                .into_iter()
                .enumerate()
                .map(|(index, exercise)| {
                    at(import_exercise(exercise), &format!("exercises[{index}]"))
                })
                .collect::<Result<_, _>>()?,
            lists: value.lists.into_iter().map(import_list).collect(),
        };
        pack.validate()?;
        Ok(pack)
    }
}

impl TryFrom<domain::Pack> for dto::Pack {
    type Error = Vec<shared::FieldError>;

    fn try_from(value: domain::Pack) -> Result<Self, Self::Error> {
        value.validate()?;
        Ok(Self {
            manifest: value.manifest.into(),
            exercises: value
                .exercises
                .into_iter()
                .enumerate()
                .map(|(index, exercise)| at(exercise.try_into(), &format!("exercises[{index}]")))
                .collect::<Result<_, _>>()?,
            lists: value.lists.into_iter().map(Into::into).collect(),
        })
    }
}
