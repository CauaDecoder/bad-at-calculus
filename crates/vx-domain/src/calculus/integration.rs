//! Definite integrals (adaptive Simpson) and Riemann sums for visualization.

use super::{Interval, RealFunction};
use crate::DomainError;

const SIMPSON_TOLERANCE: f64 = 1e-10;
const SIMPSON_MAX_DEPTH: u32 = 40;
const SIMPSON_ERROR_FACTOR: f64 = 15.0;
const MAX_RIEMANN_PIECES: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DefiniteIntegral {
    pub value: f64,
    pub estimated_error: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiemannMethod {
    Left,
    Right,
    Midpoint,
    Trapezoid,
}

/// One rectangle (or trapezoid) of a Riemann sum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RiemannPiece {
    pub start: f64,
    pub end: f64,
    pub start_height: f64,
    pub end_height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RiemannSum {
    pub method: RiemannMethod,
    pub value: f64,
    pub pieces: Vec<RiemannPiece>,
}

struct SimpsonPanel {
    start: f64,
    end: f64,
    estimate: f64,
}

impl RealFunction {
    /// ∫ₐᵇ f(x) dx. Fails if `f` is undefined somewhere in the interval or the
    /// integral does not converge (e.g. `1/x²` across 0).
    pub fn integrate(&self, interval: Interval) -> Result<DefiniteIntegral, DomainError> {
        let panel = self.panel(interval.start(), interval.end())?;
        self.refine(panel, SIMPSON_TOLERANCE, SIMPSON_MAX_DEPTH)
    }

    pub fn riemann_sum(
        &self,
        interval: Interval,
        piece_count: usize,
        method: RiemannMethod,
    ) -> Result<RiemannSum, DomainError> {
        if !(1..=MAX_RIEMANN_PIECES).contains(&piece_count) {
            return Err(DomainError::InvalidSampleCount(piece_count));
        }
        let width = interval.length() / piece_count as f64;
        let pieces = (0..piece_count)
            .map(|index| {
                let start = interval.start() + width * index as f64;
                self.riemann_piece(start, start + width, method)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let value = pieces
            .iter()
            .map(|piece| (piece.start_height + piece.end_height) / 2.0 * (piece.end - piece.start))
            .sum();
        Ok(RiemannSum {
            method,
            value,
            pieces,
        })
    }

    fn riemann_piece(
        &self,
        start: f64,
        end: f64,
        method: RiemannMethod,
    ) -> Result<RiemannPiece, DomainError> {
        let (start_height, end_height) = match method {
            RiemannMethod::Left => {
                let height = self.value_at(start)?;
                (height, height)
            }
            RiemannMethod::Right => {
                let height = self.value_at(end)?;
                (height, height)
            }
            RiemannMethod::Midpoint => {
                let height = self.value_at((start + end) / 2.0)?;
                (height, height)
            }
            RiemannMethod::Trapezoid => (self.value_at(start)?, self.value_at(end)?),
        };
        Ok(RiemannPiece {
            start,
            end,
            start_height,
            end_height,
        })
    }

    fn panel(&self, start: f64, end: f64) -> Result<SimpsonPanel, DomainError> {
        let middle = (start + end) / 2.0;
        let (f_start, f_middle, f_end) = (
            self.value_at(start)?,
            self.value_at(middle)?,
            self.value_at(end)?,
        );
        Ok(SimpsonPanel {
            start,
            end,
            estimate: (end - start) / 6.0 * (f_start + 4.0 * f_middle + f_end),
        })
    }

    /// Splits the panel in two until Simpson's estimate stops changing.
    fn refine(
        &self,
        panel: SimpsonPanel,
        tolerance: f64,
        depth_left: u32,
    ) -> Result<DefiniteIntegral, DomainError> {
        let middle = (panel.start + panel.end) / 2.0;
        let left = self.panel(panel.start, middle)?;
        let right = self.panel(middle, panel.end)?;
        let refined = left.estimate + right.estimate;
        let difference = refined - panel.estimate;

        if difference.abs() <= SIMPSON_ERROR_FACTOR * tolerance {
            return Ok(DefiniteIntegral {
                value: refined + difference / SIMPSON_ERROR_FACTOR,
                estimated_error: difference.abs() / SIMPSON_ERROR_FACTOR,
            });
        }
        if depth_left == 0 {
            return Err(DomainError::NotIntegrable);
        }
        let half_tolerance = tolerance / 2.0;
        let left_part = self.refine(left, half_tolerance, depth_left - 1)?;
        let right_part = self.refine(right, half_tolerance, depth_left - 1)?;
        Ok(DefiniteIntegral {
            value: left_part.value + right_part.value,
            estimated_error: left_part.estimated_error + right_part.estimated_error,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Expr, MathFunction};

    fn x() -> Expr {
        Expr::variable("x")
    }

    #[test]
    fn integral_of_x_squared_from_0_to_3_is_9() -> Result<(), DomainError> {
        let parabola = RealFunction::new("x", Expr::power(x(), Expr::Number(2.0)));
        let integral = parabola.integrate(Interval::new(0.0, 3.0)?)?;
        assert!((integral.value - 9.0).abs() < 1e-9);
        Ok(())
    }

    #[test]
    fn integral_of_sine_over_half_period_is_2() -> Result<(), DomainError> {
        let sine = RealFunction::new("x", Expr::call(MathFunction::Sin, x()));
        let integral = sine.integrate(Interval::new(0.0, std::f64::consts::PI)?)?;
        assert!((integral.value - 2.0).abs() < 1e-9);
        Ok(())
    }

    #[test]
    fn diverging_integral_is_rejected() -> Result<(), DomainError> {
        let spike = RealFunction::new(
            "x",
            Expr::divide(Expr::Number(1.0), Expr::power(x(), Expr::Number(2.0))),
        );
        assert!(spike.integrate(Interval::new(-1.0, 1.0)?).is_err());
        Ok(())
    }

    #[test]
    fn riemann_sums_approach_the_integral() -> Result<(), DomainError> {
        let parabola = RealFunction::new("x", Expr::power(x(), Expr::Number(2.0)));
        let interval = Interval::new(0.0, 3.0)?;
        let left = parabola.riemann_sum(interval, 3, RiemannMethod::Left)?;
        assert!((left.value - 5.0).abs() < 1e-12); // 0 + 1 + 4
        let right = parabola.riemann_sum(interval, 3, RiemannMethod::Right)?;
        assert!((right.value - 14.0).abs() < 1e-12); // 1 + 4 + 9
        let fine = parabola.riemann_sum(interval, 1000, RiemannMethod::Midpoint)?;
        assert!((fine.value - 9.0).abs() < 1e-4);
        Ok(())
    }
}
