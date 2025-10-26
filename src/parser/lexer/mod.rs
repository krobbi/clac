#[cfg(test)]
mod tests;

mod lex_error;
mod scanner;
mod token;

pub use self::{lex_error::LexError, token::Token};

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

    /// Reads the next number [`Token`] after consuming its first digit
    /// [`char`].
    fn read_number(&mut self) -> Token {
        self.scanner.eat_while(is_char_digit);

        if self.scanner.eat('.') {
            self.scanner.eat_while(is_char_digit);
        }

        let number = self.scanner.lexeme();
        let number = number.parse().expect("lexeme should be a valid float");
        Token::Number(number)
    }
}

/// Returns `true` if a [`char`] is a digit.
fn is_char_digit(char: char) -> bool {
    char.is_ascii_digit()
}
