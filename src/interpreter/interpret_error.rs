use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] caught while interpreting [`Ir`][crate::ir::Ir].
#[derive(Debug)]
pub enum InterpretError {
    /// An invalid type of [`Value`][crate::ir::Value] was used for an
    /// operation.
    InvalidType,

    /// A division by zero was attempted.
    DivideByZero,
}

impl Error for InterpretError {}

impl Display for InterpretError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidType => f.write_str("type error"),
            Self::DivideByZero => f.write_str("cannot divide by zero"),
        }
    }
}
