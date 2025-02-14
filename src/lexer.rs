use std::str;

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
    pub fn next(&mut self) -> Token {
        let char = loop {
            match self.advance() {
                None => return Token::End,
                Some(char) if char.is_whitespace() => continue,
                Some(char) => break char,
            }
        };

        match char {
            '0'..='9' => self.next_number(char),
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            _ => Token::End, // TODO: Handle this case as an error instead.
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

/// Returns a digit character converted to a number value.
fn digit_to_value(digit: char) -> f64 {
    f64::from(u32::from(digit) - u32::from('0'))
}
