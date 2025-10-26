use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use super::lexer::LexError;

/// An [`Error`] caught while parsing an [`Ast`][crate::ast::Ast].
#[derive(Debug)]
pub enum ParseError {
    /// A [`LexError`] was encountered.
    Lex(LexError),

    /// A generic syntax error.
    // TODO: Replace this error with more specific errors.
    Generic,
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
            Self::Generic => None,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lex(error) => error.fmt(f),
            Self::Generic => write!(f, "syntax error"),
        }
    }
}
