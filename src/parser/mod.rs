mod lexer;
mod parse_error;

use std::mem;

use crate::ast::Ast;

use self::{
    lexer::{LexError, Lexer, Token, TokenType},
    parse_error::ParseError,
};

/// Parses an [`Ast`] from source code. This function returns a [`ParseError`]
/// if an [`Ast`] could not be parsed.
pub fn parse_source(source: &str) -> Result<Ast, ParseError> {
    let mut parser = Parser::try_new(source)?;
    parser.parse_ast()
}

/// A structure that parses an [`Ast`] from source code.
struct Parser<'a> {
    /// The [`Lexer`] for reading [`Token`]s from source code.
    lexer: Lexer<'a>,

    /// The next [`Token`].
    next_token: Token,
}

impl<'a> Parser<'a> {
    /// Creates a new [`Parser`] from source code to be parsed. This function
    /// returns a [`LexError`] if a valid first [`Token`] could not be read.
    fn try_new(source: &'a str) -> Result<Self, LexError> {
        let mut lexer = Lexer::new(source);
        let next_token = lexer.bump()?;
        Ok(Self { lexer, next_token })
    }

    /// Parses an [`Ast`]. This function returns a [`ParseError`] if an [`Ast`]
    /// could not be parsed.
    fn parse_ast(&mut self) -> Result<Ast, ParseError> {
        self.expect(TokenType::Eof)?;
        Ok(Ast)
    }

    /// Returns the next [`Token`] without consuming it.
    fn peek(&self) -> &Token {
        &self.next_token
    }

    /// Consumes the next [`Token`]. This function returns a [`LexError`] if a
    /// valid following [`Token`] could not be read.
    fn bump(&mut self) -> Result<Token, LexError> {
        let following_token = self.lexer.bump()?;
        Ok(mem::replace(&mut self.next_token, following_token))
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns `true` if a [`Token`] was consumed and returns a
    /// [`LexError`] if a valid following [`Token`] could not be read.
    fn eat(&mut self, expected: TokenType) -> Result<bool, LexError> {
        let is_match = self.peek().as_type() == expected;

        if is_match {
            self.bump()?;
        }

        Ok(is_match)
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns a [`ParseError`] if the next [`Token`] does not
    /// match the expected [`TokenType`].
    fn expect(&mut self, expected: TokenType) -> Result<(), ParseError> {
        if self.eat(expected)? {
            Ok(())
        } else {
            let actual = self.bump()?;
            Err(ParseError::UnexpectedToken(expected, actual))
        }
    }
}
