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

    /// A non-function value was called.
    CalledNonFunction,

    /// A function was called with the incorrect number of arguments.
    IncorrectCallArity,
}

impl Error for InterpretError {}

impl Display for InterpretError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidType => "type error",
            Self::DivideByZero => "cannot divide by zero",
            Self::CalledNonFunction => "only functions can be called",
            Self::IncorrectCallArity => "incorrect number of arguments for function call",
        };

        f.write_str(message)
    }
}
