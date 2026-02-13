use thiserror::Error;

use crate::{interpreter::InterpretError, lower::LowerError, parse::ParseError};

/// An error caught while running Clac.
#[derive(Debug, Error)]
#[repr(transparent)]
#[error("Error: {0}")]
pub struct ClacError(#[source] Box<Inner>);

impl<T: Into<Inner>> From<T> for ClacError {
    #[cold]
    fn from(value: T) -> Self {
        Self(value.into().into())
    }
}

/// A kind of [`ClacError`].
#[derive(Debug, Error)]
enum Inner {
    /// A [`ParseError`].
    #[error(transparent)]
    Parse(#[from] ParseError),

    /// A [`LowerError`].
    #[error(transparent)]
    Lower(#[from] LowerError),

    /// An [`InterpretError`].
    #[error(transparent)]
    Interpret(#[from] InterpretError),
}
