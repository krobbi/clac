use std::{error, fmt};

/// An error encountered at runtime.
#[derive(Debug)]
pub enum RuntimeError {
    /// An undefined variable was evaluated.
    UndefinedVariable(String),

    /// A non-variable assignment target was assigned to.
    NonVariableAssignment,

    /// Void was used as an argument.
    VoidArgument,
}

impl error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
            Self::NonVariableAssignment => f.write_str("cannot assign to a non-variable"),
            Self::VoidArgument => f.write_str("cannot use void as an argument"),
        }
    }
}
