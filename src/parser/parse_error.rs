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

    /// A [`Token`] that did not match an expected [`TokenType`] was
    /// encountered.
    UnexpectedToken(TokenType, Token),

    /// A [`Token`] that does not begin an expected [`Expr`][crate::ast::Expr]
    /// was encountered.
    ExpectedExpr(Token),
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
            _ => None,
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
            Self::ExpectedExpr(token) => write!(f, "expected an expression, got {token}"),
        }
    }
}
