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

    /// An invalid target was assigned to.
    InvalidAssignTarget,

    /// A function was defined without an identifier name.
    InvalidFunctionName,

    /// A function was defined without an identifier parameter.
    InvalidParam,

    /// A function was defined with a duplicate parameter.
    DuplicateParam(String),

    /// A variable that is already defined was defined again.
    AlreadyDefinedVariable(String),

    /// An undefined variable was used.
    UndefinedVariable(String),
}

impl Error for ResolveError {}

impl Display for ResolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::VoidArgument => f.write_str("void cannot be used as an argument"),
            Self::InvalidAssignTarget => {
                f.write_str("can only assign to variables and function signatures")
            }
            Self::InvalidFunctionName => f.write_str("function names must be identifiers"),
            Self::InvalidParam => f.write_str("function parameters must be identifiers"),
            Self::DuplicateParam(param) => write!(f, "function parameter '{param}' is duplicated"),
            Self::AlreadyDefinedVariable(name) => write!(f, "variable '{name}' is already defined"),
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
        }
    }
}
