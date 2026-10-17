use vx_domain::{Constant, MathFunction, Relation};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Variable(String),
    Constant(Constant),
    Function(MathFunction),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    /// `²` or `³`, typed as a shortcut for `^2` / `^3`.
    Superscript(f64),
    LeftParen,
    RightParen,
    Bar,
    /// `{` opens a piecewise function: `{x < 1: x^2; x >= 1: 2x}`.
    LeftBrace,
    RightBrace,
    Colon,
    /// `;` or `,` between branches of a piecewise function.
    Separator,
    Relation(Relation),
    End,
}

impl TokenKind {
    /// Tokens that may follow another term to form an implicit product: `2x`, `3(x+1)`.
    pub fn starts_term(&self) -> bool {
        matches!(
            self,
            Self::Number(_)
                | Self::Variable(_)
                | Self::Constant(_)
                | Self::Function(_)
                | Self::LeftParen
        )
    }

    pub fn describe(&self) -> String {
        match self {
            Self::Number(value) => format!("number {value}"),
            Self::Variable(name) => format!("`{name}`"),
            Self::Constant(Constant::Pi) => "`pi`".to_owned(),
            Self::Constant(Constant::E) => "`e`".to_owned(),
            Self::Function(function) => format!("`{}`", function.name()),
            Self::Plus => "`+`".to_owned(),
            Self::Minus => "`-`".to_owned(),
            Self::Star => "`*`".to_owned(),
            Self::Slash => "`/`".to_owned(),
            Self::Caret => "`^`".to_owned(),
            Self::Superscript(_) => "superscript".to_owned(),
            Self::LeftParen => "`(`".to_owned(),
            Self::RightParen => "`)`".to_owned(),
            Self::Bar => "`|`".to_owned(),
            Self::LeftBrace => "`{`".to_owned(),
            Self::RightBrace => "`}`".to_owned(),
            Self::Colon => "`:`".to_owned(),
            Self::Separator => "`;`".to_owned(),
            Self::Relation(_) => "comparison".to_owned(),
            Self::End => "end of expression".to_owned(),
        }
    }
}

/// A token and its position (in characters) inside the source text.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: usize,
}
