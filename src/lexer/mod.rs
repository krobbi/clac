mod token;

pub use crate::lexer::token::Token;

use std::{iter::Peekable, str::Chars};

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

    /// Reads the next [`Token`].
    pub fn read_token(&mut self) -> Token {
        self.skip_whitespace();

        let Some(char) = self.chars.next() else {
            return Token::Eof;
        };

        match char {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            _ => todo!("lex error handling"),
        }
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
}
