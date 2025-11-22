use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An [`Error`] encountered while resolving an [`Ast`][crate::ast::Ast] to
/// [`Hir`][crate::hir::Hir].
#[derive(Debug)]
pub enum ResolveError {
    /// A statement was used in an area where an expression was expected.
    UsedStmt(ExprArea),

    /// A tuple was used as a standalone value.
    TupleValue,

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
            Self::UsedStmt(area) => {
                let message = match area {
                    ExprArea::Paren => "statements cannot be used inside parentheses",
                    ExprArea::Operand => "statements cannot be used as operands",
                    ExprArea::AssignSource => "statements cannot be assigned to variables",
                    ExprArea::FunctionBody => "functions must return a value",
                    ExprArea::Callee => "statements cannot be called",
                    ExprArea::Arg => "statements cannot be used as function arguments",
                };

                f.write_str(message)
            }
            Self::TupleValue => f.write_str("tuple values are not supported"),
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

/// An area where an expression must be used instead of a statement.
#[derive(Clone, Copy, Debug)]
pub enum ExprArea {
    /// Inside parentheses.
    Paren,

    /// An operand.
    Operand,

    /// An assignment source.
    AssignSource,

    /// A function body.
    FunctionBody,

    /// A callee.
    Callee,

    /// An argument.
    Arg,
}
