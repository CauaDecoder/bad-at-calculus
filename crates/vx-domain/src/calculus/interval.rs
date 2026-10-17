use crate::DomainError;

/// Closed interval `[start, end]` on the real line, with `start < end`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    start: f64,
    end: f64,
}

impl Interval {
    pub fn new(start: f64, end: f64) -> Result<Self, DomainError> {
        let is_valid = start.is_finite() && end.is_finite() && start < end;
        if !is_valid {
            return Err(DomainError::InvalidInterval { start, end });
        }
        Ok(Self { start, end })
    }

    pub fn start(&self) -> f64 {
        self.start
    }

    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn length(&self) -> f64 {
        self.end - self.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_reversed_bounds() {
        assert!(Interval::new(1.0, -1.0).is_err());
    }

    #[test]
    fn rejects_infinite_bounds() {
        assert!(Interval::new(0.0, f64::INFINITY).is_err());
    }
}
