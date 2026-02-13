use thiserror::Error;

use crate::{compilation::CompilationError, interpreter::InterpretError, parsing::ParseError};

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
    ParseError(#[from] ParseError),

    /// A [`CompilationError`].
    #[error(transparent)]
    Compilation(#[from] CompilationError),

    /// An [`InterpretError`].
    #[error(transparent)]
    Interpretation(#[from] InterpretError),
}
