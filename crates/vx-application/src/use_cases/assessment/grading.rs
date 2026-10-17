use super::input::{delimited, identifiers, located, pairs, validate_ids, validate_pairs, Segment};
use super::*;
use vx_domain::assessment::{
    combine_verdicts, compare_choices, compare_equations, compare_pairs, match_set,
};

impl CompiledAnswer {
    pub(super) fn check(
        &self,
        parser: &impl ExpressionParser,
        source: &str,
    ) -> Result<Comparison, InputIssue> {
        match self {
            Self::Numeric(_, _, unit) => {
                Ok(self.compare(&parse_checked(parser, numeric_source(source, unit), &[])?))
            }
            Self::Expression { variables, .. } => {
                Ok(self.compare(&parse_checked(parser, source, variables)?))
            }
            Self::Equation {
                expected,
                variables,
                settings,
            } => Ok(compare_equations(
                expected,
                &equation(parser, source, variables)?,
                settings,
            )),
            Self::Set(values) => check_set(values, parser, &delimited(source, ('{', '}'))?),
            Self::Tuple(values) => check_ordered(
                values,
                parser,
                &delimited(source, ('(', ')'))?,
                "response.values",
            ),
            Self::Matrix(rows) => check_matrix(rows, parser, source),
            Self::Choice {
                options,
                correct,
                multiple,
            } => {
                let selected = identifiers(source)?;
                validate_ids(&selected, options)?;
                if !*multiple && selected.len() > 1 {
                    return Err(issue("Single choice accepts at most one identifier"));
                }
                Ok(plain(compare_choices(correct, &selected)))
            }
            Self::Matching {
                left,
                right,
                pairs: expected,
            } => {
                let submitted = pairs(source)?;
                validate_pairs(&submitted, left, right)?;
                Ok(plain(compare_pairs(expected, &submitted)))
            }
            Self::SelfGraded => Ok(plain(Verdict::SelfGraded)),
        }
    }
}

fn plain(verdict: Verdict) -> Comparison {
    Comparison {
        verdict,
        sampling: None,
    }
}

fn check_ordered(
    values: &[CompiledAnswer],
    parser: &impl ExpressionParser,
    parts: &[Segment<'_>],
    field: &str,
) -> Result<Comparison, InputIssue> {
    let mut verdicts = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        let result = if let Some(value) = values.get(index) {
            value.check(parser, part.source)
        } else {
            parse_checked(parser, part.source, &[]).map(|_| plain(Verdict::Incorrect))
        };
        verdicts.push(
            result
                .map_err(|error| located(error, &format!("{field}[{index}]"), part.offset))?
                .verdict,
        );
    }
    if values.len() != parts.len() {
        return Ok(plain(Verdict::Incorrect));
    }
    Ok(plain(combine_verdicts(verdicts)))
}

fn check_set(
    values: &[CompiledAnswer],
    parser: &impl ExpressionParser,
    parts: &[Segment<'_>],
) -> Result<Comparison, InputIssue> {
    let mut edges = vec![vec![Verdict::Incorrect; parts.len()]; values.len()];
    for (column, part) in parts.iter().enumerate() {
        let mut valid = false;
        let mut error = issue("No scalar specification accepts this input");
        for (row, value) in values.iter().enumerate() {
            match value.check(parser, part.source) {
                Ok(comparison) => {
                    valid = true;
                    edges[row][column] = comparison.verdict;
                }
                Err(cause) => error = cause,
            }
        }
        if values.is_empty() {
            parse_checked(parser, part.source, &[])?;
            valid = true;
        }
        if !valid {
            return Err(located(
                error,
                &format!("response.values[{column}]"),
                part.offset,
            ));
        }
    }
    if values.len() != parts.len() {
        return Ok(plain(Verdict::Incorrect));
    }
    Ok(plain(match_set(&edges)))
}

fn check_matrix(
    rows: &[Vec<CompiledAnswer>],
    parser: &impl ExpressionParser,
    source: &str,
) -> Result<Comparison, InputIssue> {
    let parts = delimited(source, ('[', ']'))?;
    let mut verdicts = vec![];
    let mut width = None;
    let mut total = 0;
    for (index, row) in parts.iter().enumerate() {
        let mut elements = delimited(row.source, ('[', ']'))
            .map_err(|error| located(error, &format!("response.rows[{index}]"), row.offset))?;
        total += elements.len();
        if total > 64 {
            return Err(issue("At most 64 matrix elements can be graded"));
        }
        if elements.is_empty() || width.is_some_and(|width| width != elements.len()) {
            return Err(issue("A matrix must have nonempty rectangular rows"));
        }
        width = Some(elements.len());
        for element in &mut elements {
            element.offset += row.offset;
        }
        verdicts.push(
            check_ordered(
                rows.get(index).map_or(&[], Vec::as_slice),
                parser,
                &elements,
                &format!("response.rows[{index}]"),
            )?
            .verdict,
        );
    }
    if parts.is_empty() {
        return Err(issue("A matrix must have at least one row"));
    }
    if parts.len() != rows.len() {
        return Ok(plain(Verdict::Incorrect));
    }
    Ok(plain(combine_verdicts(verdicts)))
}
