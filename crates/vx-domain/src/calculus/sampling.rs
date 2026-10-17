//! Turns a continuous function into discrete samples a renderer can draw.
//!
//! Points where `f` or `f'` is undefined are kept as `None`, so the renderer
//! can break the curve there (e.g. `1/x` at `x = 0`).

use super::{Interval, RealFunction};
use crate::DomainError;

const MIN_SAMPLE_COUNT: usize = 2;
const MAX_SAMPLE_COUNT: usize = 20_000;

/// Values of `f` and `f'` at evenly spaced points of an interval.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSamples {
    pub xs: Vec<f64>,
    pub values: Vec<Option<f64>>,
    pub slopes: Vec<Option<f64>>,
}

impl RealFunction {
    pub fn sample(
        &self,
        interval: Interval,
        sample_count: usize,
    ) -> Result<FunctionSamples, DomainError> {
        let xs = evenly_spaced_points(interval, sample_count)?;
        let values = xs.iter().map(|&x| self.value_at(x).ok()).collect();
        let slopes = xs.iter().map(|&x| self.derivative_at(x).ok()).collect();
        Ok(FunctionSamples { xs, values, slopes })
    }
}

fn evenly_spaced_points(interval: Interval, count: usize) -> Result<Vec<f64>, DomainError> {
    if !(MIN_SAMPLE_COUNT..=MAX_SAMPLE_COUNT).contains(&count) {
        return Err(DomainError::InvalidSampleCount(count));
    }
    let step = interval.length() / (count - 1) as f64;
    Ok((0..count)
        .map(|index| interval.start() + step * index as f64)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinaryOperator, Expr};

    fn reciprocal() -> RealFunction {
        let expression = Expr::binary(
            BinaryOperator::Divide,
            Expr::Number(1.0),
            Expr::variable("x"),
        );
        RealFunction::new("x", expression)
    }

    #[test]
    fn samples_include_both_ends_of_interval() -> Result<(), DomainError> {
        let identity = RealFunction::new("x", Expr::variable("x"));
        let samples = identity.sample(Interval::new(-1.0, 1.0)?, 3)?;
        assert_eq!(samples.xs, vec![-1.0, 0.0, 1.0]);
        assert_eq!(samples.values, vec![Some(-1.0), Some(0.0), Some(1.0)]);
        Ok(())
    }

    #[test]
    fn undefined_points_become_gaps() -> Result<(), DomainError> {
        let samples = reciprocal().sample(Interval::new(-1.0, 1.0)?, 3)?;
        assert_eq!(samples.values[1], None);
        Ok(())
    }

    #[test]
    fn rejects_too_few_samples() -> Result<(), DomainError> {
        let result = reciprocal().sample(Interval::new(-1.0, 1.0)?, 1);
        assert_eq!(result, Err(DomainError::InvalidSampleCount(1)));
        Ok(())
    }
}
