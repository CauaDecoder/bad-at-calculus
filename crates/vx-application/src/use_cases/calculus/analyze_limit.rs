use vx_domain::calculus::{LimitAnalysis, LimitTarget};

use super::{evaluate_number, parse_function};
use crate::{ports::ExpressionParser, ApplicationError};

#[derive(Debug, Clone, PartialEq)]
pub enum LimitTargetRequest {
    /// The point as typed: `3`, `-1/2`, `pi/2`.
    Finite(String),
    PositiveInfinity,
    NegativeInfinity,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LimitRequest {
    pub expression: String,
    pub variable: String,
    pub target: LimitTargetRequest,
}

/// "What happens to f(x) as x approaches a? Is f continuous there?"
pub struct AnalyzeLimit<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> AnalyzeLimit<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &LimitRequest) -> Result<LimitAnalysis, ApplicationError> {
        let function = parse_function(&self.parser, &request.expression, &request.variable)?;
        let target = match &request.target {
            LimitTargetRequest::Finite(point) => {
                LimitTarget::Finite(evaluate_number(&self.parser, point, "point")?)
            }
            LimitTargetRequest::PositiveInfinity => LimitTarget::PositiveInfinity,
            LimitTargetRequest::NegativeInfinity => LimitTarget::NegativeInfinity,
        };
        Ok(function.analyze_limit(target))
    }
}

#[cfg(test)]
mod tests {
    use vx_domain::calculus::LimitValue;

    use super::*;
    use crate::use_cases::test_support::{FakeParserReturningXSquared, NumberParser};

    #[test]
    fn limit_of_x_squared_at_infinity_is_infinite() -> Result<(), ApplicationError> {
        let request = LimitRequest {
            expression: "x^2".into(),
            variable: "x".into(),
            target: LimitTargetRequest::PositiveInfinity,
        };
        let analysis = AnalyzeLimit::new(FakeParserReturningXSquared).execute(&request)?;
        assert_eq!(analysis.limit, LimitValue::PositiveInfinity);
        Ok(())
    }

    #[test]
    fn invalid_point_is_reported_on_its_field() {
        let request = LimitRequest {
            expression: "x^2".into(),
            variable: "x".into(),
            target: LimitTargetRequest::Finite("x".into()),
        };
        let result = AnalyzeLimit::new(NumberParser).execute(&request);
        assert!(
            matches!(result, Err(ApplicationError::InvalidField { field, .. }) if field == "point")
        );
    }
}
