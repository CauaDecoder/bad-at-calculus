use mv_domain::calculus::{CriticalPoint, Interval};

use super::parse_function;
use crate::{ports::ExpressionParser, ApplicationError};

/// Input for [`FindCriticalPoints`]: `f(x) = expression` scanned over `[start, end]`.
#[derive(Debug, Clone, PartialEq)]
pub struct CriticalPointsRequest {
    pub expression: String,
    pub variable: String,
    pub start: f64,
    pub end: f64,
}

/// "Where is the tangent horizontal in the visible range?"
pub struct FindCriticalPoints<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> FindCriticalPoints<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(
        &self,
        request: &CriticalPointsRequest,
    ) -> Result<Vec<CriticalPoint>, ApplicationError> {
        let function = parse_function(&self.parser, &request.expression, &request.variable)?;
        Ok(function.critical_points(Interval::new(request.start, request.end)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::FakeParserReturningXSquared;

    #[test]
    fn parabola_has_its_vertex_as_critical_point() -> Result<(), ApplicationError> {
        let request = CriticalPointsRequest {
            expression: "x^2".into(),
            variable: "x".into(),
            start: -2.0,
            end: 3.0,
        };
        let points = FindCriticalPoints::new(FakeParserReturningXSquared).execute(&request)?;
        assert_eq!(points.len(), 1);
        assert!(points[0].x.abs() < 1e-6);
        Ok(())
    }
}
