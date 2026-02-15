use thiserror::Error;

use super::LexError;

/// A kind of [`LexError`].
#[derive(Debug, Error)]
pub enum ErrorKind {
    /// A [`char`] which does not begin a [`Token`][crate::tokens::Token] was
    /// encountered.
    #[error("unexpected character '{}'", .0.escape_debug())]
    UnexpectedChar(char),

    /// A bitwise and (`&`) operator was encountered.
    #[error("the '&' operator is not supported, did you mean '&&'?")]
    BitwiseAnd,

    /// A bitwise or (`|`) operator was encountered.
    #[error("the '|' operator is not supported, did you mean '||'?")]
    BitwiseOr,
}

impl From<ErrorKind> for LexError {
    #[cold]
    fn from(value: ErrorKind) -> Self {
        Self(value)
    }
}
