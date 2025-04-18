use std::{error, fmt, iter, str};

use crate::{token::Token, value::Value};

/// A structure that generates a stream of tokens from source code.
pub struct Lexer<'a> {
    /// The character iterator.
    chars: iter::Peekable<str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    /// Returns the next token from the token stream.
    fn next_token(&mut self) -> Result<Token, LexError> {
        let first_char = loop {
            match self.chars.next() {
                None => return Ok(Token::Eof),
                Some(c) if c.is_whitespace() => {}
                Some(c) => break c,
            }
        };

        match first_char {
            c if c.is_ascii_digit() => Ok(self.number(c)),
            c if is_ident_start(c) => Ok(self.ident(c)),
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '=' => Ok(Token::Eq),
            ',' => Ok(Token::Comma),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            c => Err(LexError::NonToken(c)),
        }
    }

    /// Creates a new number literal token from its first digit.
    fn number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();

        while let Some(c) = self.chars.next_if(char::is_ascii_digit) {
            number.push(c);
        }

        if self.chars.next_if_eq(&'.').is_some() {
            number.push('.');

            while let Some(c) = self.chars.next_if(char::is_ascii_digit) {
                number.push(c);
            }
        }

        Token::Literal(Value::Number(number.parse().unwrap()))
    }

    /// Creates a new identifier token from its first character.
    fn ident(&mut self, first_char: char) -> Token {
        let mut name = first_char.to_string();

        while let Some(c) = self.chars.next_if(|c| is_ident_continue(*c)) {
            name.push(c);
        }

        Token::Ident(name)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_token())
    }
}

/// A syntax error encountered while lexing source code.
#[derive(Debug, Clone)]
pub enum LexError {
    /// A character was encountered that does not begin a token.
    NonToken(char),
}

impl error::Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NonToken(c) => write!(f, "unexpected character '{}'", c.escape_default()),
        }
    }
}

/// Returns whether a character is a valid identifier start.
fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

/// Returns whether a character is a valid identifier continuation.
fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || c.is_ascii_digit()
}
