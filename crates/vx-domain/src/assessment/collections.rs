use super::Verdict;
use std::collections::{BTreeMap, BTreeSet};

/// Combines ordered component outcomes without inventing partial scores.
pub fn combine_verdicts(verdicts: impl IntoIterator<Item = Verdict>) -> Verdict {
    let mut result = Verdict::Correct;
    for verdict in verdicts {
        match verdict {
            Verdict::Incorrect => return Verdict::Incorrect,
            Verdict::Indeterminate | Verdict::SelfGraded => result = Verdict::Indeterminate,
            Verdict::Correct => {}
        }
    }
    result
}

/// Finds a bijection, including augmenting paths when tolerant matches overlap.
/// Unknown edges yield indeterminate only if they could complete a full matching.
pub fn match_set(edges: &[Vec<Verdict>]) -> Verdict {
    if edges.iter().any(|row| row.len() != edges.len()) {
        return Verdict::Incorrect;
    }
    if perfect_matching(edges, |verdict| verdict == Verdict::Correct) {
        Verdict::Correct
    } else if perfect_matching(edges, |verdict| {
        matches!(verdict, Verdict::Correct | Verdict::Indeterminate)
    }) {
        Verdict::Indeterminate
    } else {
        Verdict::Incorrect
    }
}

fn perfect_matching(edges: &[Vec<Verdict>], allowed: fn(Verdict) -> bool) -> bool {
    let mut owners = vec![None; edges.len()];
    (0..edges.len()).all(|row| {
        augment(
            row,
            edges,
            &mut owners,
            (&mut vec![false; edges.len()], allowed),
        )
    })
}

fn augment(
    row: usize,
    edges: &[Vec<Verdict>],
    owners: &mut [Option<usize>],
    search: (&mut [bool], fn(Verdict) -> bool),
) -> bool {
    let (visited, allowed) = search;
    for column in 0..owners.len() {
        if visited[column] || !allowed(edges[row][column]) {
            continue;
        }
        visited[column] = true;
        if owners[column]
            .is_none_or(|previous| augment(previous, edges, owners, (visited, allowed)))
        {
            owners[column] = Some(row);
            return true;
        }
    }
    false
}

/// Objective choices are compared as sets, not as the order of selected IDs.
pub fn compare_choices(expected: &[String], response: &[String]) -> Verdict {
    if expected.iter().collect::<BTreeSet<_>>() == response.iter().collect::<BTreeSet<_>>() {
        Verdict::Correct
    } else {
        Verdict::Incorrect
    }
}

/// Matching is all-or-nothing until product defines a partial-credit policy.
pub fn compare_pairs(
    expected: &BTreeMap<String, String>,
    response: &BTreeMap<String, String>,
) -> Verdict {
    if expected == response {
        Verdict::Correct
    } else {
        Verdict::Incorrect
    }
}
