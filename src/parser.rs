use std::{error, fmt, iter};

use crate::{
    lexer::{LexError, Lexer},
    token::Token,
};

/// Parses an expression from source code.
pub fn parse_source(source: &str) -> Result<String, ParseError> {
    Parser::new(source).parse_atom()
}

/// A syntax error encountered while parsing.
#[derive(Debug)]
pub enum ParseError {
    /// An error caused by a lexing error.
    Lex(LexError),

    /// A token was encountered that does not begin an expected expression.
    NonExpression(Token),
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Lex(e) => Some(e),
            Self::NonExpression(_) => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lex(e) => e.fmt(f),
            Self::NonExpression(t) => write!(f, "expected an expression, got {t}"),
        }
    }
}

impl From<LexError> for ParseError {
    fn from(value: LexError) -> Self {
        Self::Lex(value)
    }
}

/// A structure that generates expressions from source code.
struct Parser<'a> {
    lexer: iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from source code.
    fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
        }
    }

    /// Parses an atom expression.
    fn parse_atom(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Token::Literal(value) => Ok(value.to_string()),
            t => Err(ParseError::NonExpression(t)),
        }
    }

    /// Returns the next token.
    fn next(&mut self) -> Result<Token, LexError> {
        self.lexer.next().unwrap()
    }
}
