use vx_domain::{
    calculus::{differentiate, Derivation},
    Expr,
};

use crate::{ports::ExpressionParser, ApplicationError};

#[derive(Debug, Clone, PartialEq)]
pub struct DerivativeReport {
    pub function: Expr,
    /// f'(x) with every rule applied, top-down.
    pub derivation: Derivation,
    /// f''(x), useful for concavity and exercises like `d²y/dx²`.
    pub second_derivative: Expr,
}

/// "Show me f'(x) and how to get there."
pub struct DifferentiateExpression<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> DifferentiateExpression<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(
        &self,
        source: &str,
        variable: &str,
    ) -> Result<DerivativeReport, ApplicationError> {
        let function = self.parser.parse(source)?;
        let derivation = differentiate(&function, variable);
        let second_derivative = differentiate(&derivation.derivative, variable).derivative;
        Ok(DerivativeReport {
            function,
            derivation,
            second_derivative,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::FakeParserReturningXSquared;

    #[test]
    fn derivatives_of_x_squared() -> Result<(), ApplicationError> {
        let report =
            DifferentiateExpression::new(FakeParserReturningXSquared).execute("x^2", "x")?;
        assert_eq!(report.derivation.derivative.to_latex(), "2 x");
        assert_eq!(report.second_derivative, Expr::Number(2.0));
        Ok(())
    }
}
