use mv_domain::calculus::{FunctionSamples, Interval};

use super::parse_function;
use crate::{ports::ExpressionParser, ApplicationError};

/// Input for [`SampleFunction`]: `f(x) = expression` over `[start, end]`.
#[derive(Debug, Clone, PartialEq)]
pub struct SampleFunctionRequest {
    pub expression: String,
    pub variable: String,
    pub start: f64,
    pub end: f64,
    pub sample_count: usize,
}

/// "Give me the points to draw f and f' on the visible range."
pub struct SampleFunction<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> SampleFunction<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(
        &self,
        request: &SampleFunctionRequest,
    ) -> Result<FunctionSamples, ApplicationError> {
        let function = parse_function(&self.parser, &request.expression, &request.variable)?;
        let interval = Interval::new(request.start, request.end)?;
        Ok(function.sample(interval, request.sample_count)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::FakeParserReturningXSquared;

    fn request(start: f64, end: f64) -> SampleFunctionRequest {
        SampleFunctionRequest {
            expression: "x^2".into(),
            variable: "x".into(),
            start,
            end,
            sample_count: 3,
        }
    }

    #[test]
    fn samples_values_of_x_squared() -> Result<(), ApplicationError> {
        let samples =
            SampleFunction::new(FakeParserReturningXSquared).execute(&request(-2.0, 2.0))?;
        assert_eq!(samples.values, vec![Some(4.0), Some(0.0), Some(4.0)]);
        Ok(())
    }

    #[test]
    fn reversed_range_is_a_math_error() {
        let result = SampleFunction::new(FakeParserReturningXSquared).execute(&request(2.0, -2.0));
        assert!(matches!(result, Err(ApplicationError::Math(_))));
    }
}
