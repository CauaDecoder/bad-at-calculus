//! Small, bounded submission grammar. Semicolons leave decimal commas unambiguous.
use super::*;

pub(super) struct Segment<'a> {
    pub source: &'a str,
    pub offset: usize,
}

pub(super) fn issue(message: &str) -> InputIssue {
    InputIssue {
        field: "response".into(),
        message: message.into(),
        position: Some(0),
    }
}

pub(super) fn located(mut error: InputIssue, field: &str, offset: usize) -> InputIssue {
    error.field = field.into();
    error.position = error.position.map(|position| position + offset);
    error
}

pub(super) fn delimited(
    source: &str,
    delimiters: (char, char),
) -> Result<Vec<Segment<'_>>, InputIssue> {
    let trimmed = source.trim();
    let offset = source[..source.len() - source.trim_start().len()]
        .chars()
        .count();
    let (open, close) = delimiters;
    let body = trimmed
        .strip_prefix(open)
        .and_then(|text| text.strip_suffix(close))
        .ok_or_else(|| {
            issue(&format!(
                "Use `{open}...{close}` with semicolon-separated elements"
            ))
        })?;
    if body.trim().is_empty() {
        return Ok(vec![]);
    }
    let mut items = split(body, ';')?;
    for item in &mut items {
        item.offset += offset + 1;
    }
    if items.len() > 32 {
        return Err(issue("At most 32 elements per collection or row"));
    }
    Ok(items)
}

fn split(source: &str, delimiter: char) -> Result<Vec<Segment<'_>>, InputIssue> {
    if source.len() > 65_536 {
        return Err(issue("Collection input exceeds 65536 bytes"));
    }
    let mut stack = Vec::new();
    let (mut quoted, mut escaped) = (false, false);
    let (mut start, mut offset) = (0, 0);
    let mut parts = vec![];
    for (position, (byte, character)) in source.char_indices().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        if quoted && character == '\\' {
            escaped = true;
            continue;
        }
        if character == '"' {
            quoted = !quoted;
            continue;
        }
        if quoted {
            continue;
        }
        match character {
            '(' | '[' | '{' => stack.push(character),
            ')' | ']' | '}' => {
                let expected = match character {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                if stack.pop() != Some(expected) {
                    return Err(InputIssue {
                        position: Some(position),
                        ..issue("Unbalanced collection delimiter")
                    });
                }
            }
            _ => {}
        }
        if stack.len() > 128 {
            return Err(issue("Collection nesting exceeds 128 levels"));
        }
        if character == delimiter && stack.is_empty() {
            parts.push(Segment {
                source: &source[start..byte],
                offset,
            });
            start = byte + character.len_utf8();
            offset = position + 1;
        }
    }
    if quoted || !stack.is_empty() {
        return Err(issue("Unclosed quote or collection delimiter"));
    }
    parts.push(Segment {
        source: &source[start..],
        offset,
    });
    Ok(parts)
}

pub(super) fn equation(
    parser: &impl ExpressionParser,
    source: &str,
    variables: &[String],
) -> Result<Expr, InputIssue> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(issue("Input exceeds 4096 bytes"));
    }
    let sides = split(source, '=')?;
    if sides.len() != 2 {
        return Err(InputIssue {
            position: Some(source.chars().count()),
            ..issue("An equation needs exactly one equality sign")
        });
    }
    let left = parse_checked(parser, sides[0].source, variables)?;
    let right = parse_checked(parser, sides[1].source, variables)
        .map_err(|error| located(error, "response", sides[1].offset))?;
    Ok(Expr::subtract(left, right))
}

pub(super) fn identifiers(source: &str) -> Result<Vec<String>, InputIssue> {
    let ids = delimited(source, ('[', ']'))?
        .iter()
        .map(|segment| identifier(segment.source))
        .collect::<Result<Vec<_>, _>>()?;
    let unique: std::collections::BTreeSet<_> = ids.iter().collect();
    if unique.len() != ids.len() {
        return Err(issue("Repeated option identifier"));
    }
    Ok(ids)
}

fn identifier(source: &str) -> Result<String, InputIssue> {
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return Err(issue("Empty option identifier"));
    }
    if trimmed.starts_with('"') {
        let body = trimmed
            .strip_prefix('"')
            .and_then(|text| text.strip_suffix('"'))
            .ok_or_else(|| issue("Unclosed quoted identifier"))?;
        let mut chars = body.chars();
        let mut result = String::new();
        while let Some(character) = chars.next() {
            result.push(match character {
                '\\' => match chars.next() {
                    Some('n') => '\n',
                    Some('r') => '\r',
                    Some('t') => '\t',
                    Some('"') => '"',
                    Some('\\') => '\\',
                    _ => return Err(issue("Invalid identifier escape")),
                },
                '"' => return Err(issue("Unescaped identifier quote")),
                other => other,
            });
        }
        if result.is_empty() {
            return Err(issue("Empty option identifier"));
        }
        Ok(result)
    } else if trimmed.contains([':', ';', '"', '[', ']', '{', '}', '(', ')']) {
        Err(issue("Quote identifiers containing delimiters"))
    } else {
        Ok(trimmed.into())
    }
}

pub(super) fn pairs(source: &str) -> Result<BTreeMap<String, String>, InputIssue> {
    let mut result = BTreeMap::new();
    for segment in delimited(source, ('{', '}'))? {
        let pair = split(segment.source, ':')?;
        if pair.len() != 2 {
            return Err(issue("Use `left:right` for each matching pair"));
        }
        let left = identifier(pair[0].source)?;
        let right = identifier(pair[1].source)?;
        if result.insert(left, right).is_some() {
            return Err(issue("Repeated left-side identifier"));
        }
    }
    Ok(result)
}

pub(super) fn feedback_answer(
    template: &AnswerSpec,
    source: &str,
) -> Result<AnswerSpec, InputIssue> {
    let mut result = template.clone();
    match &mut result {
        AnswerSpec::Numeric(spec) => spec.value = source.into(),
        AnswerSpec::Expression(spec) => spec.expected = source.into(),
        AnswerSpec::Equation(spec) => spec.expected = source.into(),
        AnswerSpec::Set { values, .. } => replace_values(values, source, ('{', '}'))?,
        AnswerSpec::Tuple { values, .. } => replace_values(values, source, ('(', ')'))?,
        AnswerSpec::Matrix { rows, .. } => {
            let parts = delimited(source, ('[', ']'))?;
            if rows.len() != parts.len() {
                return Err(issue("Feedback must use the answer's matrix shape"));
            }
            for (row, part) in rows.iter_mut().zip(parts) {
                replace_values(row, part.source, ('[', ']'))?;
            }
        }
        AnswerSpec::Choice {
            options,
            correct,
            multiple,
            ..
        } => {
            *correct = identifiers(source)?;
            validate_ids(correct, options)?;
            if !*multiple && correct.len() > 1 {
                return Err(issue("Single choice accepts at most one identifier"));
            }
        }
        AnswerSpec::Matching {
            left,
            right,
            pairs: expected,
            ..
        } => {
            *expected = pairs(source)?;
            validate_pairs(expected, left, right)?;
        }
        AnswerSpec::SelfGraded { .. } => {}
    }
    Ok(result)
}

fn replace_values(
    values: &mut [ScalarAnswerSpec],
    source: &str,
    delimiters: (char, char),
) -> Result<(), InputIssue> {
    let parts = delimited(source, delimiters)?;
    if parts.len() != values.len() {
        return Err(issue(
            "Feedback must have one element per scalar specification",
        ));
    }
    for (value, part) in values.iter_mut().zip(parts) {
        match value {
            ScalarAnswerSpec::Numeric(spec) => spec.value = part.source.into(),
            ScalarAnswerSpec::Expression(spec) => spec.expected = part.source.into(),
        }
    }
    Ok(())
}

pub(super) fn validate_ids(ids: &[String], options: &[AnswerOption]) -> Result<(), InputIssue> {
    if ids
        .iter()
        .any(|id| !options.iter().any(|option| &option.id == id))
    {
        Err(issue("Unknown option identifier"))
    } else {
        Ok(())
    }
}

pub(super) fn validate_pairs(
    pairs: &BTreeMap<String, String>,
    left: &[AnswerOption],
    right: &[AnswerOption],
) -> Result<(), InputIssue> {
    validate_ids(&pairs.keys().cloned().collect::<Vec<_>>(), left)?;
    validate_ids(&pairs.values().cloned().collect::<Vec<_>>(), right)
}
