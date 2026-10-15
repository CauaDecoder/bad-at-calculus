//! Text → [`Expr`] adapter: a hand-written tokenizer + Pratt parser.
//!
//! Accepts what students actually type: `x^2`, `2x`, `x²`, `3(x+1)`,
//! `sin x`, `sqrt(x)`, `|x|`, `e^x`, `pi`, `2,5` and an optional
//! `f(x) =` / `y =` prefix. See `docs/03-arquitetura/motor-matematico.md`.

mod pratt;
mod token;
mod tokenizer;

use mv_application::ports::{ExpressionParser, ParseError};
use mv_domain::Expr;

#[derive(Debug, Default, Clone, Copy)]
pub struct PrattParser;

impl ExpressionParser for PrattParser {
    fn parse(&self, source: &str) -> Result<Expr, ParseError> {
        let (body, body_position) = strip_definition_prefix(source);
        let tokens = tokenizer::tokenize(body, body_position)?;
        pratt::Parser::new(tokens).parse_complete_expression()
    }
}

/// `"f(x) = x^2"` → (`" x^2"`, 6). Positions are counted in characters.
///
/// Only an `=` before any `{` counts, and never the `=` of `<=`/`>=`, so
/// conditions inside piecewise functions are left alone.
fn strip_definition_prefix(source: &str) -> (&str, usize) {
    let mut previous = None;
    for (character_index, (byte_index, character)) in source.char_indices().enumerate() {
        if character == '{' {
            break;
        }
        let is_comparison = matches!(previous, Some('<' | '>'));
        if character == '=' && !is_comparison {
            return (&source[byte_index + 1..], character_index + 1);
        }
        previous = Some(character);
    }
    (source, 0)
}

#[cfg(test)]
mod tests {
    use mv_domain::{BinaryOperator, MathFunction};

    use super::*;

    const TOLERANCE: f64 = 1e-9;

    fn value_of(source: &str, x: f64) -> Option<f64> {
        PrattParser.parse(source).ok()?.evaluate_at("x", x).ok()
    }

    fn assert_value(source: &str, x: f64, expected: f64) {
        let actual = value_of(source, x);
        let is_close = actual.is_some_and(|value| (value - expected).abs() < TOLERANCE);
        assert!(
            is_close,
            "{source} at x={x}: expected {expected}, got {actual:?}"
        );
    }

    fn error_position(source: &str) -> Option<usize> {
        PrattParser.parse(source).err().map(|error| error.position)
    }

    #[test]
    fn respects_operator_precedence() {
        assert_value("1 + 2 * 3", 0.0, 7.0);
        assert_value("(1 + 2) * 3", 0.0, 9.0);
        assert_value("8 / 4 / 2", 0.0, 1.0);
    }

    #[test]
    fn power_is_right_associative() {
        assert_value("2^3^2", 0.0, 512.0);
    }

    #[test]
    fn unary_minus_binds_looser_than_power() {
        assert_value("-x^2", 3.0, -9.0);
        assert_value("2^-1", 0.0, 0.5);
    }

    #[test]
    fn supports_implicit_multiplication() {
        assert_value("2x", 3.0, 6.0);
        assert_value("3(x+1)", 1.0, 6.0);
        assert_value("(x+1)(x-1)", 3.0, 8.0);
        assert_value("2x^2", 3.0, 18.0);
    }

    #[test]
    fn supports_superscripts() {
        assert_value("x²", 3.0, 9.0);
        assert_value("x³ - 3x + 4", 2.0, 6.0);
    }

    #[test]
    fn supports_functions_with_and_without_parentheses() {
        assert_value("sin(x)", 0.0, 0.0);
        assert_value("sin x^2", 2.0, 4.0_f64.sin());
        assert_value("sin(x)^2 + cos(x)^2", 1.2, 1.0);
        assert_value("sqrt(x)", 9.0, 3.0);
    }

    #[test]
    fn supports_absolute_value_bars() {
        assert_value("|x|", -4.0, 4.0);
        assert_value("|x - 5| + 1", 2.0, 4.0);
    }

    #[test]
    fn supports_constants() {
        assert_value("e^x", 1.0, std::f64::consts::E);
        assert_value("2pi", 0.0, 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn ignores_definition_prefix() {
        assert_value("f(x) = x^2", 3.0, 9.0);
        assert_value("y = 2x + 1", 1.0, 3.0);
    }

    #[test]
    fn supports_piecewise_functions() {
        let source = "f(x) = {x < 1: x^2; x >= 1: 2x}";
        assert_value(source, 0.5, 0.25);
        assert_value(source, 3.0, 6.0);
    }

    #[test]
    fn supports_chained_conditions_and_otherwise_branch() {
        let source = "{-1 <= x < 2: 1; 0}";
        assert_value(source, -1.0, 1.0);
        assert_value(source, 2.0, 0.0);
    }

    #[test]
    fn reports_unclosed_piecewise() {
        assert_eq!(error_position("{x < 1: x"), Some(0));
    }

    #[test]
    fn evaluates_constant_expressions() {
        let value = PrattParser.parse("sqrt(2)/2").and_then(|expression| {
            expression
                .evaluate_constant()
                .map_err(|_| ParseError::new("not constant", 0))
        });
        assert!(value.is_ok_and(|value| (value - 0.5f64.sqrt()).abs() < TOLERANCE));
    }

    #[test]
    fn builds_the_expected_tree() {
        let expected = Expr::call(
            MathFunction::Sqrt,
            Expr::binary(BinaryOperator::Add, Expr::variable("x"), Expr::Number(1.0)),
        );
        assert_eq!(PrattParser.parse("sqrt(x + 1)"), Ok(expected));
    }

    #[test]
    fn reports_error_positions() {
        assert_eq!(error_position(""), Some(0));
        assert_eq!(error_position("x +"), Some(3));
        assert_eq!(error_position("(x + 1"), Some(0));
        assert_eq!(error_position("x + 1)"), Some(5));
        assert_eq!(error_position("y = x +"), Some(7));
    }
}
