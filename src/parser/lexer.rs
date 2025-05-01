use std::str::Chars;

use crate::ast::Literal;

use super::{syntax_error::SyntaxError, token::Token};

/// A structure that scans tokens from source code.
pub struct Lexer<'a> {
    /// The character iterator.
    chars: Chars<'a>,

    /// The syntax error log.
    errors: &'a mut Vec<SyntaxError>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from source code and a syntax error log.
    pub fn new(source: &'a str, errors: &'a mut Vec<SyntaxError>) -> Self {
        Self {
            chars: source.chars(),
            errors,
        }
    }

    /// Emits a syntax error to the syntax error log.
    pub fn emit_error(&mut self, error: SyntaxError) {
        self.errors.push(error);
    }

    /// Scans a token.
    pub fn scan_token(&mut self) -> Token {
        loop {
            let Some(first_char) = self.chars.next() else {
                break Token::Eof;
            };

            break match first_char {
                c if c.is_whitespace() => continue,
                c if c.is_ascii_digit() => self.scan_number(c),
                c if is_ident_start(c) => self.scan_ident(c),
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                '=' => Token::Eq,
                ',' => Token::Comma,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                c => {
                    self.emit_error(SyntaxError::UnexpectedChar(c));
                    continue;
                }
            };
        }
    }

    /// Scans a number literal token from its first digit.
    fn scan_number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();

        while self.peek().is_ascii_digit() {
            number.push(self.next());
        }

        if self.peek() == '.' {
            number.push(self.next());

            while self.peek().is_ascii_digit() {
                number.push(self.next());
            }
        }

        Token::Literal(Literal::Number(number.parse().unwrap()))
    }

    /// Scans an identifier token from its first character.
    fn scan_ident(&mut self, first_char: char) -> Token {
        let mut name = first_char.to_string();

        while is_ident_continue(self.peek()) {
            name.push(self.next());
        }

        Token::Ident(name)
    }

    /// Returns the next character without consuming it.
    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    /// Returns the next character.
    fn next(&mut self) -> char {
        self.chars.next().unwrap_or_default()
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
