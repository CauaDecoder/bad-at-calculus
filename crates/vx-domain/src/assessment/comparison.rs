use crate::Expr;

use super::{SamplingDomain, Tolerance};

const MIN_SAMPLES: u32 = 12;
const MAX_SAMPLES: u32 = 256;
const ATTEMPTS_PER_SAMPLE: u32 = 10;
const FIXED_PROBES: usize = 5;
const DEFAULT_ABS_TOLERANCE: f64 = 1e-12;
const DEFAULT_REL_TOLERANCE: f64 = 1e-9;

/// Mathematical outcome; input syntax is handled before comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Correct,
    Incorrect,
    Indeterminate,
    /// The engine does not determine correctness for a manual exercise.
    SelfGraded,
}

/// Reproducible sampling diagnostics, not a formal proof or verification evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct SamplingReport {
    pub seed: u32,
    pub domain: SamplingDomain,
    pub requested: u32,
    pub valid: u32,
    pub discarded: u32,
}

/// Result of a pure scalar comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct Comparison {
    pub verdict: Verdict,
    pub sampling: Option<SamplingReport>,
}

/// Validated sampling configuration. Constructed through [`Self::new`].
#[derive(Clone)]
pub struct SamplingSettings {
    seed: u32,
    domain: SamplingDomain,
    samples: u32,
    tolerance: Tolerance,
}

impl SamplingSettings {
    /// Bounds work without changing the structural content contract from F1.1.
    pub fn new(
        seed: u32,
        domain: SamplingDomain,
        samples: u32,
        tolerance: Tolerance,
    ) -> Option<Self> {
        let valid = (MIN_SAMPLES..=MAX_SAMPLES).contains(&samples)
            && !domain.is_empty()
            && domain
                .values()
                .all(|[a, b]| a.is_finite() && b.is_finite() && a < b)
            && tolerance.validate().is_ok();
        valid.then_some(Self {
            seed,
            domain,
            samples,
            tolerance,
        })
    }
}

/// Compares finite constants with the F1 default absolute/relative tolerances.
pub fn compare_numbers(expected: f64, response: f64, tolerance: &Tolerance) -> Comparison {
    Comparison {
        verdict: if close(expected, response, tolerance) {
            Verdict::Correct
        } else {
            Verdict::Incorrect
        },
        sampling: None,
    }
}

fn close(expected: f64, response: f64, tolerance: &Tolerance) -> bool {
    if !expected.is_finite() || !response.is_finite() {
        return false;
    }
    // Scaling first avoids overflow in both the difference and relative threshold.
    let scale = expected.abs().max(response.abs()).max(1.0);
    let difference = (expected / scale - response / scale).abs();
    difference <= tolerance.abs.unwrap_or(DEFAULT_ABS_TOLERANCE) / scale
        || difference
            <= tolerance.rel.unwrap_or(DEFAULT_REL_TOLERANCE)
                * (expected.abs().max(response.abs()) / scale)
}

/// Samples the unsimplified ASTs. Only undefined expected values may be discarded.
pub fn compare_expressions(
    expected: &Expr,
    response: &Expr,
    settings: &SamplingSettings,
) -> Comparison {
    sample_pair(expected, response, settings, ComparisonRule::Expression)
}

/// Compares equation residuals by a finite, constant, nonzero scale (ADR 0009).
pub fn compare_equations(
    expected: &Expr,
    response: &Expr,
    settings: &SamplingSettings,
) -> Comparison {
    sample_pair(expected, response, settings, ComparisonRule::Equation(None))
}

enum ComparisonRule {
    Expression,
    Equation(Option<f64>),
}

impl ComparisonRule {
    fn accepts(&mut self, expected: f64, response: f64, tolerance: &Tolerance) -> bool {
        match self {
            Self::Expression => close(expected, response, tolerance),
            Self::Equation(_) if expected == 0.0 => close(0.0, response, tolerance),
            Self::Equation(scale) => {
                let ratio = response / expected;
                if !ratio.is_finite() || ratio == 0.0 {
                    return false;
                }
                let reference = *scale.get_or_insert(ratio);
                // Normalize around one: an arbitrarily small scale must not hide a varying ratio.
                close(1.0, ratio / reference, tolerance)
            }
        }
    }

    fn counts(&self, expected: f64) -> bool {
        matches!(self, Self::Expression) || expected != 0.0
    }
}

fn sample_pair(
    expected: &Expr,
    response: &Expr,
    settings: &SamplingSettings,
    mut rule: ComparisonRule,
) -> Comparison {
    let mut report = SamplingReport {
        seed: settings.seed,
        domain: settings.domain.clone(),
        requested: settings.samples,
        valid: 0,
        discarded: 0,
    };
    let mut state = settings.seed;
    // Fixed probes catch common holes/sign errors; they do not replace random samples.
    for probe in 0..FIXED_PROBES {
        let point = probe_point(&settings.domain, probe);
        let binding = |name: &str| point.get(name).copied();
        if expected.evaluate_with(&binding).is_ok_and(|value| {
            !response
                .evaluate_with(&binding)
                .is_ok_and(|answer| rule.accepts(value, answer, &settings.tolerance))
        }) {
            return sampled(Verdict::Incorrect, report);
        }
    }
    for _ in 0..settings.samples * ATTEMPTS_PER_SAMPLE {
        let point = random_point(&settings.domain, &mut state);
        let binding = |name: &str| point.get(name).copied();
        let Ok(value) = expected.evaluate_with(&binding) else {
            report.discarded += 1;
            continue;
        };
        if !rule.counts(value) {
            if !response
                .evaluate_with(&binding)
                .is_ok_and(|answer| rule.accepts(value, answer, &settings.tolerance))
            {
                return sampled(Verdict::Incorrect, report);
            }
            report.discarded += 1;
            continue;
        }
        report.valid += 1;
        let matches = response
            .evaluate_with(&binding)
            .is_ok_and(|answer| rule.accepts(value, answer, &settings.tolerance));
        if !matches {
            return sampled(Verdict::Incorrect, report);
        }
        if report.valid == settings.samples {
            return sampled(Verdict::Correct, report);
        }
    }
    sampled(Verdict::Indeterminate, report)
}

fn sampled(verdict: Verdict, report: SamplingReport) -> Comparison {
    Comparison {
        verdict,
        sampling: Some(report),
    }
}

fn probe_point(domain: &SamplingDomain, probe: usize) -> std::collections::BTreeMap<String, f64> {
    domain
        .iter()
        .map(|(name, [a, b])| {
            let value = match probe {
                0 if *a <= 0.0 && *b >= 0.0 => 0.0,
                0 => a * 0.5 + b * 0.5,
                1 => *a,
                2 => *b,
                3 => a * 0.75 + b * 0.25,
                _ => a * 0.25 + b * 0.75,
            };
            (name.clone(), value)
        })
        .collect()
}

fn random_point(
    domain: &SamplingDomain,
    state: &mut u32,
) -> std::collections::BTreeMap<String, f64> {
    domain
        .iter()
        .map(|(name, [a, b])| {
            // Full-period 32-bit LCG with explicit wrapping, identical on native and WASM.
            *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let fraction = (f64::from(*state) + 0.5) / 4_294_967_296.0;
            (name.clone(), a * (1.0 - fraction) + b * fraction)
        })
        .collect()
}
