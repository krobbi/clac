use std::{error, fmt, iter, str};

use crate::token::Token;

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
    pub fn next(&mut self) -> Result<Token, LexError> {
        let first_char = loop {
            match self.chars.next() {
                None => return Ok(Token::Eof),
                Some(c) if c.is_whitespace() => {}
                Some(c) => break c,
            }
        };

        match first_char {
            c if c.is_ascii_digit() => Ok(self.number(c)),
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Star),
            '/' => Ok(Token::Slash),
            c => Err(LexError::CharNotTokenStart(c)),
        }
    }

    /// Returns the next number literal token from the token stream.
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

        Token::Literal(number.parse().unwrap())
    }
}

/// A syntax error encountered while lexing source code.
#[derive(Debug)]
pub enum LexError {
    /// A character was encountered that does not begin a token.
    CharNotTokenStart(char),
}

impl error::Error for LexError {}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::CharNotTokenStart(c) => {
                write!(f, "unexpected character '{}'", c.escape_default())
            }
        }
    }
}
