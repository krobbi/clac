use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] caught while reading a [`Token`][super::Token].
#[derive(Debug)]
pub enum LexError {
    /// A [`char`] that does not begin a [`Token`][super::Token] was
    /// encountered.
    UnexpectedChar(char),

    /// A bitwise and (`&`) operator was used.
    BitwiseAnd,

    /// A bitwise or (`|`) operator was used.
    BitwiseOr,
}

impl Error for LexError {}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(char) => {
                write!(f, "unexpected character '{}'", char.escape_default())
            }
            Self::BitwiseAnd => {
                f.write_str("the '&' operator is not supported, did you mean '&&'?")
            }
            Self::BitwiseOr => f.write_str("the '|' operator is not supported, did you mean '||'?"),
        }
    }
}
