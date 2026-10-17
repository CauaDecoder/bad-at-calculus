//! Points where the tangent is horizontal (`f'(x) = 0`), found numerically.
//! Corners where `f'` jumps sign (like `|x|` at 0) are reported too: they are
//! critical points in the textbook sense (`f'` does not exist there).

use super::{Interval, RealFunction};

const SCAN_STEPS: usize = 2000;
const BISECTION_ITERATIONS: usize = 60;
const MAX_CRITICAL_POINTS: usize = 50;
/// Two roots closer than this fraction of the interval are the same root.
const DUPLICATE_RATIO: f64 = 1e-6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CriticalPointKind {
    LocalMinimum,
    LocalMaximum,
    /// `f'` is zero but keeps its sign (e.g. `x³` at 0).
    Stationary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CriticalPoint {
    pub x: f64,
    pub y: f64,
    pub kind: CriticalPointKind,
}

impl RealFunction {
    /// Scans `f'` for sign changes and near-zero dips, then refines by bisection.
    pub fn critical_points(&self, interval: Interval) -> Vec<CriticalPoint> {
        let step = interval.length() / SCAN_STEPS as f64;
        let slopes: Vec<(f64, Option<f64>)> = (0..=SCAN_STEPS)
            .map(|index| {
                let x = interval.start() + step * index as f64;
                (x, self.derivative_at(x).ok())
            })
            .collect();

        let mut roots = Vec::new();
        for window in slopes.windows(3) {
            let [(x0, Some(s0)), (x1, Some(s1)), (_, Some(s2))] = window else {
                continue;
            };
            if s0.signum() != s1.signum() && *s0 != 0.0 {
                roots.extend(self.bisect_slope(*x0, *x1));
            }
            let is_touching_zero =
                s1.abs() < s0.abs() && s1.abs() < s2.abs() && s0.signum() == s2.signum();
            if is_touching_zero && s1.abs() < step {
                roots.push(*x1);
            }
        }

        roots.sort_by(f64::total_cmp);
        roots.dedup_by(|a, b| (*a - *b).abs() < interval.length() * DUPLICATE_RATIO);
        roots
            .into_iter()
            .filter_map(|x| self.classify(x, step))
            .take(MAX_CRITICAL_POINTS)
            .collect()
    }

    fn bisect_slope(&self, mut low: f64, mut high: f64) -> Option<f64> {
        let mut low_sign = self.derivative_at(low).ok()?.signum();
        for _ in 0..BISECTION_ITERATIONS {
            let middle = (low + high) / 2.0;
            let sign = self.derivative_at(middle).ok()?.signum();
            if sign == low_sign {
                low = middle;
                low_sign = sign;
            } else {
                high = middle;
            }
        }
        Some((low + high) / 2.0)
    }

    fn classify(&self, x: f64, step: f64) -> Option<CriticalPoint> {
        let y = self.value_at(x).ok()?;
        let before = self.derivative_at(x - step).ok()?;
        let after = self.derivative_at(x + step).ok()?;
        let kind = match (before < 0.0, after < 0.0) {
            (true, false) => CriticalPointKind::LocalMinimum,
            (false, true) => CriticalPointKind::LocalMaximum,
            _ => CriticalPointKind::Stationary,
        };
        Some(CriticalPoint { x, y, kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Expr;

    fn x() -> Expr {
        Expr::variable("x")
    }

    /// ED derivação, ex. 5: `x³ − 3x + 4` has horizontal tangents at x = ±1.
    fn cubic() -> RealFunction {
        let expression = Expr::sum(
            Expr::subtract(
                Expr::power(x(), Expr::Number(3.0)),
                Expr::multiply(Expr::Number(3.0), x()),
            ),
            Expr::Number(4.0),
        );
        RealFunction::new("x", expression)
    }

    #[test]
    fn finds_maximum_and_minimum_of_cubic() -> Result<(), crate::DomainError> {
        let points = cubic().critical_points(Interval::new(-3.0, 3.0)?);
        assert_eq!(points.len(), 2);
        assert!((points[0].x + 1.0).abs() < 1e-6);
        assert_eq!(points[0].kind, CriticalPointKind::LocalMaximum);
        assert!((points[1].x - 1.0).abs() < 1e-6);
        assert_eq!(points[1].kind, CriticalPointKind::LocalMinimum);
        Ok(())
    }

    #[test]
    fn x_cubed_has_a_stationary_point_at_zero() -> Result<(), crate::DomainError> {
        let x_cubed = RealFunction::new("x", Expr::power(x(), Expr::Number(3.0)));
        let points = x_cubed.critical_points(Interval::new(-2.0, 2.0)?);
        assert_eq!(points.len(), 1);
        assert!(points[0].x.abs() < 1e-2);
        assert_eq!(points[0].kind, CriticalPointKind::Stationary);
        Ok(())
    }
}
