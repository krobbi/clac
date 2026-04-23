use thiserror::Error;

use crate::{interpret::InterpretError, lower::LowerError, parse::ParseError};

/// An error caught while running Clac.
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct ClacError(Box<Kind>);

impl<E: Into<Kind>> From<E> for ClacError {
    #[cold]
    fn from(value: E) -> Self {
        Self(Box::new(value.into()))
    }
}

/// A [`ClacError`]'s kind.
#[derive(Debug, Error)]
#[error("Error: {0}")]
enum Kind {
    /// A [`ParseError`].
    Parse(#[from] ParseError),

    /// A [`LowerError`].
    Lower(#[from] LowerError),

    /// An [`InterpretError`].
    Interpret(#[from] InterpretError),
}
