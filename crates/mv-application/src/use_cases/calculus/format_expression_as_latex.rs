use crate::{ports::ExpressionParser, ApplicationError};

/// "Show what I typed as real math notation" — `x^2/2` → `\frac{x^{2}}{2}`.
pub struct FormatExpressionAsLatex<P: ExpressionParser> {
    parser: P,
}

impl<P: ExpressionParser> FormatExpressionAsLatex<P> {
    pub fn new(parser: P) -> Self {
        Self { parser }
    }

    pub fn execute(&self, source: &str) -> Result<String, ApplicationError> {
        Ok(self.parser.parse(source)?.to_latex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::test_support::FakeParserReturningXSquared;

    #[test]
    fn formats_parsed_expression() -> Result<(), ApplicationError> {
        let latex = FormatExpressionAsLatex::new(FakeParserReturningXSquared).execute("x^2")?;
        assert_eq!(latex, "x^{2}");
        Ok(())
    }
}
