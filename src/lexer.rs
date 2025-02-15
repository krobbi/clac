use std::{error, fmt, str};

use crate::token::Token;

/// A structure that creates tokens from statement source code.
pub struct Lexer<'a> {
    /// The character iterator.
    chars: str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from statement source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars(),
        }
    }

    /// Returns the next token.
    pub fn next(&mut self) -> Result<Token, LexError> {
        let char = loop {
            match self.advance() {
                None => return Ok(Token::End),
                Some(char) if char.is_whitespace() => continue,
                Some(char) => break char,
            }
        };

        match char {
            '0'..='9' => Ok(self.next_number(char)),
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '+' => Ok(Token::Add),
            '-' => Ok(Token::Subtract),
            '*' => Ok(Token::Multiply),
            '/' => Ok(Token::Divide),
            _ => Err(LexError::UnexpectedChar(char)),
        }
    }

    /// Returns the next number token from its first digit character.
    fn next_number(&mut self, digit: char) -> Token {
        let mut value = digit_to_value(digit);

        while let Some(digit @ '0'..='9') = self.peek() {
            self.advance();
            value = value * 10.0 + digit_to_value(digit);
        }

        if let Some('.') = self.peek() {
            self.advance();
            let mut divisor = 1.0;

            while let Some(digit @ '0'..='9') = self.peek() {
                self.advance();
                divisor *= 10.0;
                value += digit_to_value(digit) / divisor;
            }
        }

        Token::Number(value)
    }

    /// Returns the next character without consuming it.
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Consumes and returns the next character.
    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }
}

/// A syntax error encountered while lexing.
#[derive(Debug, Clone, Copy)]
pub enum LexError {
    /// A character was encountered that does not form a valid token.
    UnexpectedChar(char),
}

impl error::Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedChar(char) => {
                write!(f, "unexpected character '{}'", char.escape_default())
            }
        }
    }
}

/// Returns a digit character converted to a number value.
fn digit_to_value(digit: char) -> f64 {
    f64::from(u32::from(digit) - u32::from('0'))
}
