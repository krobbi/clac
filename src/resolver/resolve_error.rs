use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] caught while resolving an [`Ast`][crate::ast::Ast].
#[derive(Debug)]
pub enum ResolveError {
    /// An undefined variable was used.
    UndefinedVariable(String),
}

impl Error for ResolveError {}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
        }
    }
}
