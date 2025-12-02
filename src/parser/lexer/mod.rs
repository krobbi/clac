#[cfg(test)]
mod tests;

mod lex_error;
mod scanner;
mod token;

pub use self::{
    lex_error::LexError,
    token::{Token, TokenType},
};

use crate::ast::Literal;

use self::scanner::Scanner;

/// A structure that reads a stream of [`Token`]s from source code.
pub struct Lexer<'a> {
    /// The [`Scanner`] for reading [`char`]s from source code.
    scanner: Scanner<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` from source code to be read.
    pub fn new(source: &'a str) -> Self {
        let scanner = Scanner::new(source);
        Self { scanner }
    }

    /// Consumes the next [`Token`] from source code. This function returns a
    /// [`LexError`] if a valid [`Token`] could not be read.
    pub fn bump(&mut self) -> Result<Token, LexError> {
        self.scanner.eat_while(char::is_whitespace);
        self.scanner.begin_lexeme();

        let Some(char) = self.scanner.bump() else {
            return Ok(Token::Eof);
        };

        let token = match char {
            c if is_char_digit(c) => self.read_number(),
            c if is_char_word_start(c) => self.read_word(),
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => {
                if self.scanner.eat('>') {
                    Token::RightArrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Star,
            '/' => Token::Slash,
            '=' => Token::Eq,
            '!' => Token::Bang,
            _ => return Err(LexError::UnexpectedChar(char)),
        };

        Ok(token)
    }

    /// Reads the next number [`Token`] after consuming its first [`char`].
    fn read_number(&mut self) -> Token {
        self.scanner.eat_while(is_char_digit);

        if self.scanner.eat('.') {
            self.scanner.eat_while(is_char_digit);
        }

        let value = self.scanner.lexeme();
        let value = value.parse().expect("value should be a valid float");
        Token::Literal(Literal::Number(value))
    }

    /// Reads the next identifier or keyword [`Token`] after consuming its first
    /// [`char`].
    fn read_word(&mut self) -> Token {
        self.scanner.eat_while(is_char_word_continue);

        match self.scanner.lexeme() {
            "false" => Token::Literal(Literal::Bool(false)),
            "true" => Token::Literal(Literal::Bool(true)),
            name => Token::Ident(name.to_owned()),
        }
    }
}

/// Returns `true` if a [`char`] is a digit.
fn is_char_digit(char: char) -> bool {
    char.is_ascii_digit()
}

/// Returns `true` if a [`char`] is an identifier or keyword start.
fn is_char_word_start(char: char) -> bool {
    char.is_ascii_alphabetic() || char == '_'
}

/// Return `true` if a [`char`] is an identifier or keyword continuation.
fn is_char_word_continue(char: char) -> bool {
    is_char_word_start(char) || is_char_digit(char)
}
