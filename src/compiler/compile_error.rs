use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] caught while compiling [`Ir`][crate::ir::Ir].
#[derive(Debug)]
pub enum CompileError {
    /// An invalid target was assigned to.
    InvalidAssignTarget,

    /// A variable that is already defined was defined again.
    AlreadyDefinedVariable(String),

    /// A variable that is undefined was used.
    UndefinedVariable(String),
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAssignTarget => {
                f.write_str("can only assign to variables and function signatures")
            }
            Self::AlreadyDefinedVariable(name) => write!(f, "variable '{name}' is already defined"),
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
        }
    }
}
