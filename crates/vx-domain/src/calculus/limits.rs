//! Numerical study of limits, one side at a time, plus continuity at a point.
//!
//! Every limit is reduced to `t → 0⁺`:
//! `x → a⁺` uses `x = a + t`, `x → a⁻` uses `x = a − t`,
//! `x → +∞` uses `x = 1/t` and `x → −∞` uses `x = −1/t`.
//! Two estimates are compared and the most self-consistent wins:
//! Richardson extrapolation (great for smooth functions such as `sin(x)/x`)
//! and direct evaluation very close to the point (robust for roots such as `√x`).

use super::RealFunction;

const RICHARDSON_START: f64 = 0.1;
const RICHARDSON_POINTS: usize = 12;
const RICHARDSON_MAX_COLUMNS: usize = 6;
const DIRECT_EXPONENTS: std::ops::RangeInclusive<i32> = 1..=10;
const ACCEPTANCE_TOLERANCE: f64 = 1e-4;
const SNAP_TOLERANCE: f64 = 1e-6;
const SAME_LIMIT_TOLERANCE: f64 = 1e-4;
/// Values shown in the approach table (as in the exercise lists).
const TABLE_DISTANCES: [f64; 4] = [0.1, 0.01, 0.001, 0.0001];
/// Divergence: the last increments do not shrink below this share of the first.
const DIVERGENCE_INCREMENT_RATIO: f64 = 0.9;
const DIVERGENCE_WINDOW: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimitTarget {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimitValue {
    Finite(f64),
    PositiveInfinity,
    NegativeInfinity,
    /// Oscillates or sides disagree.
    DoesNotExist,
    /// `f` is not defined on this side (e.g. `√x` for `x → 0⁻`).
    NotDefined,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApproachSample {
    pub x: f64,
    pub value: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OneSidedLimit {
    pub value: LimitValue,
    pub samples: Vec<ApproachSample>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscontinuityKind {
    Removable,
    Jump,
    Infinite,
    Oscillating,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContinuityAnalysis {
    pub value_at_point: Option<f64>,
    pub is_continuous: bool,
    pub is_continuous_from_left: bool,
    pub is_continuous_from_right: bool,
    pub discontinuity: Option<DiscontinuityKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LimitAnalysis {
    pub target: LimitTarget,
    /// `x → a⁻` (or `x → +∞`, approached from below).
    pub from_below: Option<OneSidedLimit>,
    /// `x → a⁺` (or `x → −∞`, approached from above).
    pub from_above: Option<OneSidedLimit>,
    pub limit: LimitValue,
    /// Only for finite targets.
    pub continuity: Option<ContinuityAnalysis>,
}

impl RealFunction {
    pub fn analyze_limit(&self, target: LimitTarget) -> LimitAnalysis {
        match target {
            LimitTarget::Finite(a) => self.analyze_finite_limit(a),
            LimitTarget::PositiveInfinity => {
                let side = self.one_sided(|t| 1.0 / t);
                LimitAnalysis {
                    target,
                    limit: side.value,
                    from_below: Some(side),
                    from_above: None,
                    continuity: None,
                }
            }
            LimitTarget::NegativeInfinity => {
                let side = self.one_sided(|t| -1.0 / t);
                LimitAnalysis {
                    target,
                    limit: side.value,
                    from_below: None,
                    from_above: Some(side),
                    continuity: None,
                }
            }
        }
    }

    fn analyze_finite_limit(&self, a: f64) -> LimitAnalysis {
        let left = self.one_sided(|t| a - t);
        let right = self.one_sided(|t| a + t);
        let limit = combine_sides(left.value, right.value);
        let continuity = continuity_at(self.value_at(a).ok(), left.value, right.value, limit);
        LimitAnalysis {
            target: LimitTarget::Finite(a),
            from_below: Some(left),
            from_above: Some(right),
            limit,
            continuity: Some(continuity),
        }
    }

    fn one_sided(&self, x_at: impl Fn(f64) -> f64) -> OneSidedLimit {
        let value_at = |t: f64| self.value_at(x_at(t)).ok();
        let samples = TABLE_DISTANCES
            .iter()
            .map(|&t| ApproachSample {
                x: x_at(t),
                value: value_at(t),
            })
            .collect();
        OneSidedLimit {
            value: estimate_limit(&value_at),
            samples,
        }
    }
}

fn estimate_limit(value_at: &impl Fn(f64) -> Option<f64>) -> LimitValue {
    let direct: Vec<Option<f64>> = DIRECT_EXPONENTS
        .map(|exponent| value_at(10f64.powi(-exponent)))
        .collect();
    if direct.iter().rev().take(3).all(Option::is_none) {
        return LimitValue::NotDefined;
    }

    // Order matters: direct evaluation very close to the point can be ruined by
    // cancellation (`1 − cos x` becomes exactly 0), so it is the last resort.
    let candidates = [
        richardson_estimate(value_at).filter(|estimate| agrees_with_trend(estimate, &direct)),
        aitken_estimate(&direct),
        direct_estimate(&direct),
    ];
    let accepted = candidates.into_iter().flatten().find(|estimate| {
        estimate.value.is_finite() && estimate.relative_error() < ACCEPTANCE_TOLERANCE
    });
    match accepted {
        Some(estimate) => LimitValue::Finite(snap(estimate.value)),
        None => divergence(&direct).unwrap_or(LimitValue::DoesNotExist),
    }
}

#[derive(Debug, Clone, Copy)]
struct Estimate {
    value: f64,
    error: f64,
}

impl Estimate {
    fn relative_error(&self) -> f64 {
        self.error / (1.0 + self.value.abs())
    }
}

/// Neville/Richardson table for `t_i = t₀ / 2^i`, extrapolated to `t = 0`.
fn richardson_estimate(value_at: &impl Fn(f64) -> Option<f64>) -> Option<Estimate> {
    let values: Option<Vec<f64>> = (0..RICHARDSON_POINTS)
        .map(|index| value_at(RICHARDSON_START / 2f64.powi(index as i32)))
        .collect();
    let mut row = values?;
    let mut best: Option<Estimate> = None;

    for column in 1..=RICHARDSON_MAX_COLUMNS {
        let factor = 2f64.powi(column as i32) - 1.0;
        let next: Vec<f64> = row
            .windows(2)
            .map(|pair| pair[1] + (pair[1] - pair[0]) / factor)
            .collect();
        let (Some(&last), Some(&previous_column_last)) = (next.last(), row.last()) else {
            break;
        };
        let candidate = Estimate {
            value: last,
            error: (last - previous_column_last).abs(),
        };
        if best.is_none_or(|current| candidate.error < current.error) {
            best = Some(candidate);
        }
        row = next;
    }
    best
}

/// Richardson assumes `f(t) = L + c₁t + c₂t² + …`. For `√t` that is false and it
/// can confidently return a wrong value, so we check it against the direct values:
/// at `t = 10⁻⁴` a smooth function is already much closer to `L` than it was
/// one decade earlier.
fn agrees_with_trend(estimate: &Estimate, direct: &[Option<f64>]) -> bool {
    const TREND_INDEX: usize = 3; // t = 10⁻⁴
    const TREND_RATIO: f64 = 0.2;
    const ABSOLUTE_SLACK: f64 = 1e-7;
    let (Some(Some(previous)), Some(Some(current))) =
        (direct.get(TREND_INDEX - 1), direct.get(TREND_INDEX))
    else {
        return true;
    };
    let distance_to_estimate = (estimate.value - current).abs();
    distance_to_estimate <= TREND_RATIO * (current - previous).abs() + ABSOLUTE_SLACK
}

/// Aitken's Δ² on the direct values: exact for geometric convergence such as
/// `√t` (each step shrinks by the same ratio). Only used while converging.
fn aitken_estimate(direct: &[Option<f64>]) -> Option<Estimate> {
    let [.., Some(v0), Some(v1), Some(v2), Some(v3)] = direct else {
        return None;
    };
    let latest = aitken(*v1, *v2, *v3)?;
    let previous = aitken(*v0, *v1, *v2)?;
    Some(Estimate {
        value: latest,
        error: (latest - previous).abs(),
    })
}

fn aitken(v0: f64, v1: f64, v2: f64) -> Option<f64> {
    let (d1, d2) = (v1 - v0, v2 - v1);
    let is_converging = d1 != 0.0 && (d2 / d1).abs() < 1.0;
    let denominator = d2 - d1;
    (is_converging && denominator != 0.0).then(|| v2 - d2 * d2 / denominator)
}

fn direct_estimate(direct: &[Option<f64>]) -> Option<Estimate> {
    let [.., Some(before_last), Some(last)] = direct else {
        return None;
    };
    Some(Estimate {
        value: *last,
        error: (last - before_last).abs(),
    })
}

/// Monotone, same sign, and not slowing down → the function blows up.
fn divergence(direct: &[Option<f64>]) -> Option<LimitValue> {
    let tail: Vec<f64> = direct
        .iter()
        .rev()
        .take(DIVERGENCE_WINDOW)
        .rev()
        .copied()
        .collect::<Option<Vec<f64>>>()?;
    let increments: Vec<f64> = tail.windows(2).map(|pair| pair[1] - pair[0]).collect();
    let is_increasing = increments.iter().all(|step| *step > 0.0);
    let is_decreasing = increments.iter().all(|step| *step < 0.0);
    let (first, last) = (increments.first()?.abs(), increments.last()?.abs());
    let keeps_pace = last >= first * DIVERGENCE_INCREMENT_RATIO;
    match (is_increasing, is_decreasing, keeps_pace) {
        (true, _, true) => Some(LimitValue::PositiveInfinity),
        (_, true, true) => Some(LimitValue::NegativeInfinity),
        _ => None,
    }
}

fn snap(value: f64) -> f64 {
    let rounded = value.round();
    if (value - rounded).abs() < SNAP_TOLERANCE * value.abs().max(1.0) {
        rounded + 0.0
    } else {
        value
    }
}

fn combine_sides(left: LimitValue, right: LimitValue) -> LimitValue {
    match (left, right) {
        (LimitValue::Finite(a), LimitValue::Finite(b)) if same_limit(a, b) => {
            LimitValue::Finite(snap((a + b) / 2.0))
        }
        (LimitValue::PositiveInfinity, LimitValue::PositiveInfinity) => {
            LimitValue::PositiveInfinity
        }
        (LimitValue::NegativeInfinity, LimitValue::NegativeInfinity) => {
            LimitValue::NegativeInfinity
        }
        (LimitValue::NotDefined, LimitValue::NotDefined) => LimitValue::NotDefined,
        _ => LimitValue::DoesNotExist,
    }
}

fn continuity_at(
    value: Option<f64>,
    left: LimitValue,
    right: LimitValue,
    limit: LimitValue,
) -> ContinuityAnalysis {
    let matches_value = |side: LimitValue| match (side, value) {
        (LimitValue::Finite(side_limit), Some(value)) => same_limit(side_limit, value),
        _ => false,
    };
    let is_continuous = matches_value(limit);
    let discontinuity = if is_continuous {
        None
    } else {
        classify_discontinuity(left, right)
    };
    ContinuityAnalysis {
        value_at_point: value,
        is_continuous,
        is_continuous_from_left: matches_value(left),
        is_continuous_from_right: matches_value(right),
        discontinuity,
    }
}

fn classify_discontinuity(left: LimitValue, right: LimitValue) -> Option<DiscontinuityKind> {
    let is_infinite = |side| {
        matches!(
            side,
            LimitValue::PositiveInfinity | LimitValue::NegativeInfinity
        )
    };
    match (left, right) {
        _ if is_infinite(left) || is_infinite(right) => Some(DiscontinuityKind::Infinite),
        (LimitValue::DoesNotExist, _) | (_, LimitValue::DoesNotExist) => {
            Some(DiscontinuityKind::Oscillating)
        }
        (LimitValue::Finite(a), LimitValue::Finite(b)) if same_limit(a, b) => {
            Some(DiscontinuityKind::Removable)
        }
        (LimitValue::Finite(_), LimitValue::Finite(_)) => Some(DiscontinuityKind::Jump),
        _ => None,
    }
}

fn same_limit(a: f64, b: f64) -> bool {
    (a - b).abs() < SAME_LIMIT_TOLERANCE * (1.0 + a.abs().max(b.abs()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BinaryOperator, Comparison, Condition, Expr, MathFunction, PiecewiseBranch, Relation,
    };

    fn x() -> Expr {
        Expr::variable("x")
    }

    fn number(value: f64) -> Expr {
        Expr::Number(value)
    }

    fn function(expression: Expr) -> RealFunction {
        RealFunction::new("x", expression)
    }

    fn limit_at(expression: Expr, a: f64) -> LimitAnalysis {
        function(expression).analyze_limit(LimitTarget::Finite(a))
    }

    fn assert_finite(value: LimitValue, expected: f64) {
        let is_close =
            matches!(value, LimitValue::Finite(actual) if (actual - expected).abs() < 1e-6);
        assert!(is_close, "expected {expected}, got {value:?}");
    }

    #[test]
    fn sin_x_over_x_tends_to_one() {
        let expression = Expr::divide(Expr::call(MathFunction::Sin, x()), x());
        let analysis = limit_at(expression, 0.0);
        assert_finite(analysis.limit, 1.0);
        let continuity = analysis.continuity;
        assert_eq!(
            continuity.and_then(|c| c.discontinuity),
            Some(DiscontinuityKind::Removable)
        );
    }

    #[test]
    fn one_minus_cosine_over_x_squared_tends_to_one_half() {
        let expression = Expr::divide(
            Expr::subtract(number(1.0), Expr::call(MathFunction::Cos, x())),
            Expr::power(x(), number(2.0)),
        );
        assert_finite(limit_at(expression, 0.0).limit, 0.5);
    }

    #[test]
    fn removable_discontinuity_from_exercise_list() {
        // Lista 03, ex. 3: (x² − 2x − 3)/(x² − 9) at x = 3 → 4/6.
        let numerator = Expr::subtract(
            Expr::subtract(
                Expr::power(x(), number(2.0)),
                Expr::multiply(number(2.0), x()),
            ),
            number(3.0),
        );
        let denominator = Expr::subtract(Expr::power(x(), number(2.0)), number(9.0));
        let expression = Expr::divide(numerator, denominator);

        assert_finite(limit_at(expression.clone(), 3.0).limit, 2.0 / 3.0);
        let at_minus_three = limit_at(expression, -3.0);
        assert_eq!(
            at_minus_three.continuity.and_then(|c| c.discontinuity),
            Some(DiscontinuityKind::Infinite)
        );
    }

    #[test]
    fn one_over_x_has_different_infinite_sides() {
        let analysis = limit_at(Expr::divide(number(1.0), x()), 0.0);
        assert_eq!(
            analysis.from_below.map(|side| side.value),
            Some(LimitValue::NegativeInfinity)
        );
        assert_eq!(
            analysis.from_above.map(|side| side.value),
            Some(LimitValue::PositiveInfinity)
        );
        assert_eq!(analysis.limit, LimitValue::DoesNotExist);
    }

    #[test]
    fn square_root_is_not_defined_on_the_left_of_zero() {
        let analysis = limit_at(Expr::call(MathFunction::Sqrt, x()), 0.0);
        assert_eq!(
            analysis.from_below.map(|side| side.value),
            Some(LimitValue::NotDefined)
        );
        assert_finite(
            analysis
                .from_above
                .map_or(LimitValue::DoesNotExist, |side| side.value),
            0.0,
        );
        assert!(analysis
            .continuity
            .is_some_and(|c| c.is_continuous_from_right));
    }

    #[test]
    fn jump_discontinuity_of_piecewise_function() {
        let expression = Expr::Piecewise(vec![
            PiecewiseBranch {
                condition: Some(Condition {
                    comparisons: vec![Comparison {
                        left: x(),
                        relation: Relation::Less,
                        right: number(1.0),
                    }],
                }),
                value: x(),
            },
            PiecewiseBranch {
                condition: None,
                value: Expr::binary(BinaryOperator::Add, x(), number(2.0)),
            },
        ]);
        let analysis = limit_at(expression, 1.0);
        assert_eq!(analysis.limit, LimitValue::DoesNotExist);
        assert_eq!(
            analysis.continuity.and_then(|c| c.discontinuity),
            Some(DiscontinuityKind::Jump)
        );
    }

    #[test]
    fn limits_at_infinity() {
        let rational = Expr::divide(
            Expr::sum(
                Expr::multiply(number(3.0), Expr::power(x(), number(2.0))),
                number(1.0),
            ),
            Expr::subtract(Expr::power(x(), number(2.0)), number(5.0)),
        );
        assert_finite(
            function(rational)
                .analyze_limit(LimitTarget::PositiveInfinity)
                .limit,
            3.0,
        );

        let parabola = Expr::power(x(), number(2.0));
        assert_eq!(
            function(parabola)
                .analyze_limit(LimitTarget::NegativeInfinity)
                .limit,
            LimitValue::PositiveInfinity
        );
    }

    #[test]
    fn logarithm_goes_to_minus_infinity_at_zero() {
        let analysis = limit_at(Expr::call(MathFunction::Ln, x()), 0.0);
        assert_eq!(
            analysis.from_above.map(|side| side.value),
            Some(LimitValue::NegativeInfinity)
        );
    }

    #[test]
    fn continuous_polynomial() {
        let analysis = limit_at(Expr::power(x(), number(2.0)), 2.0);
        assert_finite(analysis.limit, 4.0);
        assert!(analysis.continuity.is_some_and(|c| c.is_continuous));
    }

    #[test]
    fn sine_of_one_over_x_oscillates() {
        let expression = Expr::call(MathFunction::Sin, Expr::divide(number(1.0), x()));
        assert_eq!(limit_at(expression, 0.0).limit, LimitValue::DoesNotExist);
    }
}
