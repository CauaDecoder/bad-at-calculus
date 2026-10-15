//! Tokens → [`Expr`] using Pratt parsing (operator binding powers).
//!
//! Binding powers (higher binds tighter), see `docs/03-arquitetura/motor-matematico.md`:
//! `+ -` 10 · `* /` and implicit product 20 · prefix `-` 30 · `^` 40 (right-assoc).

use mv_application::ports::ParseError;
use mv_domain::{BinaryOperator, Comparison, Condition, Expr, MathFunction, PiecewiseBranch};

use super::token::{Token, TokenKind};

const LOWEST_BINDING_POWER: u8 = 0;
const SUM_BINDING_POWER: (u8, u8) = (10, 11);
const PRODUCT_BINDING_POWER: (u8, u8) = (20, 21);
const PREFIX_BINDING_POWER: u8 = 30;
const POWER_BINDING_POWER: (u8, u8) = (41, 40);

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn parse_complete_expression(mut self) -> Result<Expr, ParseError> {
        if self.peek().kind == TokenKind::End {
            return Err(ParseError::new("type an expression", self.peek().position));
        }
        let expression = self.parse_expression(LOWEST_BINDING_POWER)?;
        let next = self.peek();
        if next.kind != TokenKind::End {
            return Err(unexpected(next));
        }
        Ok(expression)
    }

    fn parse_expression(&mut self, min_binding_power: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_prefix()?;

        loop {
            let next = self.peek().kind.clone();
            if let TokenKind::Superscript(exponent) = next {
                if POWER_BINDING_POWER.0 < min_binding_power {
                    break;
                }
                self.advance();
                left = Expr::binary(BinaryOperator::Power, left, Expr::Number(exponent));
                continue;
            }

            let Some((operator, (left_power, right_power))) = infix_operator(&next) else {
                break;
            };
            if left_power < min_binding_power {
                break;
            }
            if !next.starts_term() {
                self.advance();
            }
            let right = self.parse_expression(right_power)?;
            left = Expr::binary(operator, left, right);
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance();
        match token.kind {
            TokenKind::Number(value) => Ok(Expr::Number(value)),
            TokenKind::Variable(name) => Ok(Expr::Variable(name)),
            TokenKind::Constant(constant) => Ok(Expr::Constant(constant)),
            TokenKind::Minus => Ok(Expr::negate(self.parse_expression(PREFIX_BINDING_POWER)?)),
            TokenKind::Plus => self.parse_expression(PREFIX_BINDING_POWER),
            TokenKind::LeftParen => self.parse_group(TokenKind::RightParen, token.position),
            TokenKind::Bar => {
                let inner = self.parse_group(TokenKind::Bar, token.position)?;
                Ok(Expr::call(MathFunction::Abs, inner))
            }
            TokenKind::Function(function) => self.parse_call(function),
            TokenKind::LeftBrace => self.parse_piecewise(token.position),
            TokenKind::End => Err(ParseError::new("expression is incomplete", token.position)),
            _ => Err(unexpected(&token)),
        }
    }

    /// `sin(x)` takes the parenthesized group; `sin x^2` takes the next power term.
    fn parse_call(&mut self, function: MathFunction) -> Result<Expr, ParseError> {
        let argument = if self.peek().kind == TokenKind::LeftParen {
            let opening = self.advance();
            self.parse_group(TokenKind::RightParen, opening.position)?
        } else {
            self.parse_expression(PREFIX_BINDING_POWER)?
        };
        Ok(Expr::call(function, argument))
    }

    /// `{cond: value; cond: value; otherwise}` — branches separated by `;` (or `,`).
    fn parse_piecewise(&mut self, opened_at: usize) -> Result<Expr, ParseError> {
        let mut branches = Vec::new();
        loop {
            branches.push(self.parse_branch()?);
            let separator = self.advance();
            match separator.kind {
                TokenKind::Separator => continue,
                TokenKind::RightBrace => break,
                TokenKind::End => {
                    return Err(ParseError::new("missing closing `}`", opened_at));
                }
                _ => return Err(unexpected(&separator)),
            }
        }
        Ok(Expr::Piecewise(branches))
    }

    fn parse_branch(&mut self) -> Result<PiecewiseBranch, ParseError> {
        let first = self.parse_expression(LOWEST_BINDING_POWER)?;
        if !matches!(self.peek().kind, TokenKind::Relation(_)) {
            return Ok(PiecewiseBranch {
                condition: None,
                value: first,
            });
        }
        let condition = self.parse_condition(first)?;
        let colon = self.advance();
        if colon.kind != TokenKind::Colon {
            return Err(ParseError::new(
                "expected `:` between condition and value",
                colon.position,
            ));
        }
        Ok(PiecewiseBranch {
            condition: Some(condition),
            value: self.parse_expression(LOWEST_BINDING_POWER)?,
        })
    }

    /// Chained comparisons: `-1 < x <= 2` → `[-1 < x, x <= 2]`.
    fn parse_condition(&mut self, first: Expr) -> Result<Condition, ParseError> {
        let mut comparisons = Vec::new();
        let mut left = first;
        while let TokenKind::Relation(relation) = self.peek().kind {
            self.advance();
            let right = self.parse_expression(LOWEST_BINDING_POWER)?;
            comparisons.push(Comparison {
                left,
                relation,
                right: right.clone(),
            });
            left = right;
        }
        Ok(Condition { comparisons })
    }

    fn parse_group(&mut self, closing: TokenKind, opened_at: usize) -> Result<Expr, ParseError> {
        let inner = self.parse_expression(LOWEST_BINDING_POWER)?;
        if self.peek().kind != closing {
            let message = format!("missing closing {}", closing.describe());
            return Err(ParseError::new(message, opened_at));
        }
        self.advance();
        Ok(inner)
    }

    fn peek(&self) -> &Token {
        let last = self.tokens.len().saturating_sub(1);
        &self.tokens[self.cursor.min(last)]
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        self.cursor += 1;
        token
    }
}

fn infix_operator(kind: &TokenKind) -> Option<(BinaryOperator, (u8, u8))> {
    let operator = match kind {
        TokenKind::Plus => (BinaryOperator::Add, SUM_BINDING_POWER),
        TokenKind::Minus => (BinaryOperator::Subtract, SUM_BINDING_POWER),
        TokenKind::Star => (BinaryOperator::Multiply, PRODUCT_BINDING_POWER),
        TokenKind::Slash => (BinaryOperator::Divide, PRODUCT_BINDING_POWER),
        TokenKind::Caret => (BinaryOperator::Power, POWER_BINDING_POWER),
        implicit if implicit.starts_term() => (BinaryOperator::Multiply, PRODUCT_BINDING_POWER),
        _ => return None,
    };
    Some(operator)
}

fn unexpected(token: &Token) -> ParseError {
    ParseError::new(
        format!("unexpected {}", token.kind.describe()),
        token.position,
    )
}
