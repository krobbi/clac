use thiserror::Error;

use super::InterpretError;

/// A kind of [`InterpretError`].
#[derive(Debug, Error)]
pub enum ErrorKind {
    /// An invalid type was used for an operation.
    #[error("type error")]
    InvalidType,

    /// A division by zero was attempted.
    #[error("cannot divide by zero")]
    DivideByZero,

    /// A non-function was called.
    #[error("only functions can be called")]
    CalledNonFunction,

    /// A function was called with the incorrect number of arguments.
    #[error("incorrect number of arguments for function call")]
    IncorrectCallArity,
}

impl From<ErrorKind> for InterpretError {
    #[cold]
    fn from(value: ErrorKind) -> Self {
        Self(value)
    }
}
