use mv_domain::calculus::TangentLine;

use super::parse_function;
use crate::{ports::ExpressionParser, ApplicationError};

/// Input for [`ComputeTangentLine`]: `f(x) = expression`, touched at `x0`.
#[derive(Debug, Clone, PartialEq)]
pub struct TangentLineRequest {
    pub expression: String,
    pub variable: String,
    pub x0: f64,
}

/// "Show me the tangent line of f at x₀" — the first v0.1 experience.
pub struct ComputeTangentLine<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> ComputeTangentLine<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, request: &TangentLineRequest) -> Result<TangentLine, ApplicationError> {
        let function = parse_function(&self.parser, &request.expression, &request.variable)?;
        Ok(function.tangent_line_at(request.x0)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::{FailingParser, FakeParserReturningXSquared};

    fn request_at(x0: f64) -> TangentLineRequest {
        TangentLineRequest {
            expression: "x^2".into(),
            variable: "x".into(),
            x0,
        }
    }

    #[test]
    fn tangent_of_x_squared_at_2_has_slope_4() -> Result<(), ApplicationError> {
        let use_case = ComputeTangentLine::new(FakeParserReturningXSquared);
        let tangent = use_case.execute(&request_at(2.0))?;
        assert!((tangent.slope - 4.0).abs() < 1e-6);
        Ok(())
    }

    #[test]
    fn parse_failure_becomes_invalid_expression() {
        let use_case = ComputeTangentLine::new(FailingParser);
        let result = use_case.execute(&request_at(0.0));
        assert!(matches!(
            result,
            Err(ApplicationError::InvalidExpression(_))
        ));
    }
}
