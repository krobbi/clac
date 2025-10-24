mod error;
mod token;

pub use crate::lexer::token::Token;

use std::{iter::Peekable, str::Chars};

use crate::lexer::error::LexError;

/// A structure that reads a stream of [`Token`]s from source code.
pub struct Lexer<'a> {
    /// The [`Peekable`] iterator for reading [`char`]s from source code.
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` from source code to be read.
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars().peekable();
        Self { chars }
    }

    /// Reads the next [`Token`]. This function returns a [`LexError`] if a
    /// valid [`Token`] cannot be read.
    pub fn read_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let Some(char) = self.chars.next() else {
            return Ok(Token::Eof);
        };

        let token = match char {
            '0'..='9' => self.read_number(char),
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            _ => return Err(LexError::UnexpectedChar(char)),
        };

        Ok(token)
    }

    /// Advances the `Lexer` until the next [`char`] is not whitespace according
    /// to the Unicode `White_Space` property.
    fn skip_whitespace(&mut self) {
        while let Some(char) = self.chars.peek() {
            if char.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    /// Reads the next number [`Token`] from its first digit [`char`].
    fn read_number(&mut self, first_digit: char) -> Token {
        debug_assert!(
            first_digit.is_ascii_digit(),
            "`first_digit` should be matched as an ASCII digit"
        );

        let mut number = first_digit.to_string();

        while let Some(digit) = self.chars.next_if(char::is_ascii_digit) {
            number.push(digit);
        }

        if let Some(point) = self.chars.next_if_eq(&'.') {
            number.push(point);

            while let Some(digit) = self.chars.next_if(char::is_ascii_digit) {
                number.push(digit);
            }
        }

        let number = number.parse().expect("`number` should be a valid float");
        Token::Number(number)
    }
}
