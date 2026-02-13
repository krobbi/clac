use thiserror::Error;

use crate::tokens::{Token, TokenType};

use super::lexer::LexError;

/// A kind of [`ParsingError`][super::ParsingError].
#[derive(Debug, Error)]
pub enum ErrorKind {
    /// A [`LexError`].
    #[error(transparent)]
    Lexing(#[from] LexError),

    /// A [`Token`] which does not match an expected [`TokenType`] was
    /// encountered.
    #[error("expected {0}, got {1}")]
    UnexpectedToken(TokenType, Token),

    /// A [`Token`] which does not begin an expected [`Expr`][crate::ast::Expr]
    /// was encountered.
    #[error("expected an expression, got {0}")]
    ExpectedExpr(Token),

    /// A chained assignment was encountered.
    #[error("assignments cannot be chained")]
    ChainedAssignment,

    /// A chained comparison was encountered.
    #[error("comparisons cannot be chained")]
    ChainedComparison,
}
