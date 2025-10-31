use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{compiler::CompileError, parser::ParseError};

/// An [`Error`] caught while executing source code.
#[derive(Debug)]
pub enum ExecuteError {
    /// A [`ParseError`] was encountered.
    Parse(ParseError),

    /// A [`CompileError`] was encountered.
    Compile(CompileError),
}

impl From<ParseError> for ExecuteError {
    fn from(value: ParseError) -> Self {
        Self::Parse(value)
    }
}

impl From<CompileError> for ExecuteError {
    fn from(value: CompileError) -> Self {
        Self::Compile(value)
    }
}

impl Error for ExecuteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Compile(error) => Some(error),
        }
    }
}

impl Display for ExecuteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => error.fmt(f),
            Self::Compile(error) => error.fmt(f),
        }
    }
}
