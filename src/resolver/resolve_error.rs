use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] caught while resolving an [`Ast`][crate::ast::Ast].
#[derive(Debug)]
pub enum ResolveError {
    /// An invalid assignment target was used.
    InvalidAssignTarget,

    /// An undefined variable was used.
    UndefinedVariable(String),

    /// A defined variable was already defined.
    AlreadyDefinedVariable(String),
}

impl Error for ResolveError {}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAssignTarget => {
                f.write_str("can only assign to variables and function signatures")
            }
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
            Self::AlreadyDefinedVariable(name) => write!(f, "variable '{name}' is already defined"),
        }
    }
}
