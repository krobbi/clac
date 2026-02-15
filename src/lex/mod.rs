#[cfg(test)]
mod tests;

mod errors;
mod scan;

use thiserror::Error;

use crate::{ast::Literal, symbols::Symbol, tokens::Token};

use self::{errors::ErrorKind, scan::Scanner};

/// An error caught while reading a [`Token`].
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct LexError(ErrorKind);

/// A structure which reads a stream of [`Token`]s from source code.
pub struct Lexer<'src> {
    /// The [`Scanner`].
    scanner: Scanner<'src>,
}

impl<'src> Lexer<'src> {
    /// Creates a new `Lexer` from source code.
    pub fn new(source: &'src str) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    /// Returns the next [`Token`]. This function returns a [`LexError`] if a
    /// [`Token`] could not be read.
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.scanner.eat_while(char::is_whitespace);
        self.scanner.begin_lexeme();

        let Some(char) = self.scanner.bump() else {
            return Ok(Token::Eof);
        };

        let token = match char {
            c if is_char_digit(c) => self.next_number_token(),
            c if is_char_word_start(c) => self.next_word_token(),
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => {
                if self.scanner.eat('>') {
                    Token::MinusGreater
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Star,
            '/' => Token::Slash,
            '^' => Token::Caret,
            '=' => {
                if self.scanner.eat('=') {
                    Token::EqualsEquals
                } else {
                    Token::Equals
                }
            }
            '!' => {
                if self.scanner.eat('=') {
                    Token::BangEquals
                } else {
                    Token::Bang
                }
            }
            '<' => {
                if self.scanner.eat('=') {
                    Token::LessEquals
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.scanner.eat('=') {
                    Token::GreaterEquals
                } else {
                    Token::Greater
                }
            }
            '&' => {
                if self.scanner.eat('&') {
                    Token::AndAnd
                } else {
                    return Err(ErrorKind::BitwiseAnd.into());
                }
            }
            '|' => {
                if self.scanner.eat('|') {
                    Token::PipePipe
                } else {
                    return Err(ErrorKind::BitwiseOr.into());
                }
            }
            '?' => Token::Question,
            ':' => Token::Colon,
            _ => return Err(ErrorKind::UnexpectedChar(char).into()),
        };

        Ok(token)
    }

    /// Returns the next number [`Token`] after consuming its first [`char`].
    fn next_number_token(&mut self) -> Token {
        self.scanner.eat_while(is_char_digit);

        if self.scanner.eat('.') {
            self.scanner.eat_while(is_char_digit);
        }

        let value = self.scanner.lexeme();
        let value = value.parse().expect("value should be a valid float");
        Token::Literal(Literal::Number(value))
    }

    /// Returns the next keyword or identifier [`Token`] after consuming its
    /// first [`char`].
    fn next_word_token(&mut self) -> Token {
        self.scanner.eat_while(is_char_word_continue);

        match self.scanner.lexeme() {
            "false" => Token::Literal(Literal::Bool(false)),
            "true" => Token::Literal(Literal::Bool(true)),
            name => Token::Ident(Symbol::intern(name)),
        }
    }
}

/// Returns [`true`] if a [`char`] is a digit.
const fn is_char_digit(char: char) -> bool {
    char.is_ascii_digit()
}

/// Returns [`true`] if a [`char`] is a keyword or identifier start.
const fn is_char_word_start(char: char) -> bool {
    char.is_ascii_alphabetic() || char == '_'
}

/// Return [`true`] if a [`char`] is a keyword or identifier continuation.
const fn is_char_word_continue(char: char) -> bool {
    is_char_word_start(char) || is_char_digit(char)
}
