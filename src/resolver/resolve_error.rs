use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] encountered while resolving an [`Ast`][crate::ast::Ast] to
/// [`Hir`][crate::hir::Hir].
#[derive(Debug)]
pub enum ResolveError {
    /// Void was used as an argument.
    VoidArgument,
}

impl Error for ResolveError {}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::VoidArgument => f.write_str("void cannot be used as an argument"),
        }
    }
}
