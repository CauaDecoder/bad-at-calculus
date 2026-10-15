//! Text → tokens. Letters are split greedily into known names, so
//! `sinx`, `2pix` and `xsinx` read the way students write them.

use mv_application::ports::ParseError;
use mv_domain::{Constant, MathFunction, Relation};

use super::token::{Token, TokenKind};

const PI_NAME: &str = "pi";

pub fn tokenize(source: &str, first_position: usize) -> Result<Vec<Token>, ParseError> {
    let characters: Vec<char> = source.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;

    while let Some(&character) = characters.get(index) {
        let position = first_position + index;
        if character.is_whitespace() {
            index += 1;
        } else if character.is_ascii_digit() || character == '.' {
            let (value, length) = read_number(&characters[index..], position)?;
            tokens.push(Token {
                kind: TokenKind::Number(value),
                position,
            });
            index += length;
        } else if is_name_character(character) {
            let length = count_while(&characters[index..], is_name_character);
            let word: String = characters[index..index + length].iter().collect();
            tokens.extend(split_word(&word, position));
            index += length;
        } else if let Some(kind) = read_two_character_relation(character, characters.get(index + 1))
        {
            tokens.push(Token { kind, position });
            index += 2;
        } else {
            tokens.push(Token {
                kind: read_symbol(character, position)?,
                position,
            });
            index += 1;
        }
    }

    tokens.push(Token {
        kind: TokenKind::End,
        position: first_position + characters.len(),
    });
    Ok(tokens)
}

fn is_name_character(character: char) -> bool {
    character.is_alphabetic() && character != 'π'
}

fn count_while(characters: &[char], predicate: impl Fn(char) -> bool) -> usize {
    characters
        .iter()
        .take_while(|&&character| predicate(character))
        .count()
}

/// Reads `12`, `2.5` or `2,5` (Brazilian decimal comma).
fn read_number(characters: &[char], position: usize) -> Result<(f64, usize), ParseError> {
    let integer_length = count_while(characters, |character| character.is_ascii_digit());
    let has_separator = matches!(characters.get(integer_length), Some('.' | ','));
    let fraction_length = if has_separator {
        count_while(&characters[integer_length + 1..], |character| {
            character.is_ascii_digit()
        })
    } else {
        0
    };

    let is_bare_separator = integer_length == 0 && fraction_length == 0;
    if is_bare_separator {
        return Err(ParseError::new("expected a number", position));
    }

    let separator_length = usize::from(has_separator && fraction_length > 0);
    let length = integer_length + separator_length + fraction_length;
    let text: String = characters[..length]
        .iter()
        .map(|&character| if character == ',' { '.' } else { character })
        .collect();
    let value = text
        .parse()
        .map_err(|_| ParseError::new(format!("invalid number `{text}`"), position))?;
    Ok((value, length))
}

fn split_word(word: &str, position: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut rest = word;
    let mut offset = 0;

    while !rest.is_empty() {
        let (kind, length) = longest_known_name(rest).unwrap_or_else(|| single_letter(rest));
        tokens.push(Token {
            kind,
            position: position + offset,
        });
        let byte_length: usize = rest.chars().take(length).map(char::len_utf8).sum();
        rest = &rest[byte_length..];
        offset += length;
    }
    tokens
}

fn longest_known_name(text: &str) -> Option<(TokenKind, usize)> {
    let functions = MathFunction::ALL
        .into_iter()
        .map(|function| (TokenKind::Function(function), function.name()));
    let constants = std::iter::once((TokenKind::Constant(Constant::Pi), PI_NAME));

    functions
        .chain(constants)
        .filter(|(_, name)| text.starts_with(name))
        .max_by_key(|(_, name)| name.len())
        .map(|(kind, name)| (kind, name.chars().count()))
}

fn single_letter(text: &str) -> (TokenKind, usize) {
    let letter: String = text.chars().take(1).collect();
    let kind = if letter == "e" {
        TokenKind::Constant(Constant::E)
    } else {
        TokenKind::Variable(letter)
    };
    (kind, 1)
}

/// `<=` and `>=`, typed with two keys.
fn read_two_character_relation(first: char, second: Option<&char>) -> Option<TokenKind> {
    match (first, second) {
        ('<', Some('=')) => Some(TokenKind::Relation(Relation::LessOrEqual)),
        ('>', Some('=')) => Some(TokenKind::Relation(Relation::GreaterOrEqual)),
        _ => None,
    }
}

fn read_symbol(character: char, position: usize) -> Result<TokenKind, ParseError> {
    let kind = match character {
        '+' => TokenKind::Plus,
        '-' | '−' => TokenKind::Minus,
        '*' | '·' | '×' => TokenKind::Star,
        '/' | '÷' => TokenKind::Slash,
        '^' => TokenKind::Caret,
        '²' => TokenKind::Superscript(2.0),
        '³' => TokenKind::Superscript(3.0),
        '(' => TokenKind::LeftParen,
        ')' => TokenKind::RightParen,
        '|' => TokenKind::Bar,
        'π' => TokenKind::Constant(Constant::Pi),
        '√' => TokenKind::Function(MathFunction::Sqrt),
        '{' => TokenKind::LeftBrace,
        '}' => TokenKind::RightBrace,
        ':' => TokenKind::Colon,
        ',' | ';' => TokenKind::Separator,
        '<' => TokenKind::Relation(Relation::Less),
        '>' => TokenKind::Relation(Relation::Greater),
        '≤' => TokenKind::Relation(Relation::LessOrEqual),
        '≥' => TokenKind::Relation(Relation::GreaterOrEqual),
        '=' => TokenKind::Relation(Relation::Equal),
        other => {
            return Err(ParseError::new(
                format!("unexpected character `{other}`"),
                position,
            ))
        }
    };
    Ok(kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(source: &str) -> Vec<TokenKind> {
        tokenize(source, 0)
            .map(|tokens| tokens.into_iter().map(|token| token.kind).collect())
            .unwrap_or_default()
    }

    #[test]
    fn splits_function_glued_to_variable() {
        assert_eq!(
            kinds("sinx"),
            vec![
                TokenKind::Function(MathFunction::Sin),
                TokenKind::Variable("x".into()),
                TokenKind::End
            ]
        );
    }

    #[test]
    fn accepts_decimal_comma() {
        assert_eq!(kinds("2,5"), vec![TokenKind::Number(2.5), TokenKind::End]);
    }

    #[test]
    fn exp_wins_over_constant_e() {
        assert_eq!(kinds("exp")[0], TokenKind::Function(MathFunction::Exp));
    }

    #[test]
    fn reports_position_of_unknown_character() {
        assert_eq!(
            tokenize("x + $", 0),
            Err(ParseError::new("unexpected character `$`", 4))
        );
    }
}
