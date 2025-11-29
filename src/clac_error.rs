use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{interpreter::InterpretError, parser::ParseError, resolver::ResolveError};

/// An [`Error`] caught while running Clac.
#[derive(Debug)]
pub enum ClacError {
    /// A [`ParseError`] was encountered.
    Parse(ParseError),

    /// A [`ResolveError`] was encountered.
    Resolve(ResolveError),

    /// An [`InterpretError`] was encountered.
    Interpret(InterpretError),
}

impl From<ParseError> for ClacError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<ResolveError> for ClacError {
    fn from(value: ResolveError) -> Self {
        Self::Resolve(value)
    }
}

impl From<InterpretError> for ClacError {
    fn from(value: InterpretError) -> Self {
        Self::Interpret(value)
    }
}

impl Error for ClacError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Resolve(error) => Some(error),
            Self::Interpret(error) => Some(error),
        }
    }
}

impl Display for ClacError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => error.fmt(f),
            Self::Resolve(error) => error.fmt(f),
            Self::Interpret(error) => error.fmt(f),
        }
    }
}
