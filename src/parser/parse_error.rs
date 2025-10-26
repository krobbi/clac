use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use super::lexer::{LexError, Token, TokenType};

/// An [`Error`] caught while parsing an [`Ast`][crate::ast::Ast].
#[derive(Debug)]
pub enum ParseError {
    /// A [`LexError`] was encountered.
    Lex(LexError),

    /// A [`Token`] was encountered that did not match an expected
    /// [`TokenType`].
    UnexpectedToken(TokenType, Token),
}

impl From<LexError> for ParseError {
    fn from(value: LexError) -> Self {
        Self::Lex(value)
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lex(error) => Some(error),
            Self::UnexpectedToken(..) => None,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex(error) => error.fmt(f),
            Self::UnexpectedToken(expected, actual) => {
                write!(f, "expected {expected}, got {actual}")
            }
        }
    }
}
