use thiserror::Error;

use crate::symbols::Symbol;

/// A kind of [`LoweringError`][super::LoweringError].
#[derive(Debug, Error)]
pub enum ErrorKind {
    /// A statement was used in an area where an expression was expected.
    #[error(transparent)]
    UsedStmt(#[from] ExprArea),

    /// A tuple was used as a standalone value.
    #[error("tuple values are not supported")]
    TupleValue,

    /// An invalid target was assigned to.
    #[error("can only assign to variables and function signatures")]
    InvalidAssignTarget,

    /// A function was defined without an identifier name.
    #[error("function names must be identifiers")]
    InvalidFunctionName,

    /// A function was defined without an identifier parameter.
    #[error("function parameters must be identifiers")]
    InvalidParam,

    /// A function was defined with a duplicate parameter.
    #[error("function parameter '{0}' is duplicated")]
    DuplicateParam(Symbol),

    /// A variable that is already defined was defined again.
    #[error("variable '{0}' is already defined")]
    AlreadyDefinedVariable(Symbol),

    /// An undefined variable was used.
    #[error("variable '{0}' is undefined")]
    UndefinedVariable(Symbol),
}

/// An area where an expression must be used instead of a statement.
#[derive(Clone, Copy, Debug, Error)]
pub enum ExprArea {
    /// An assignment source.
    #[error("statements cannot be assigned to variables")]
    AssignSource,

    /// Inside parentheses.
    #[error("statements cannot be used inside parentheses")]
    Paren,

    /// A function body.
    #[error("functions must return a value")]
    FunctionBody,

    /// A callee.
    #[error("statements cannot be called")]
    Callee,

    /// An argument.
    #[error("statements cannot be used as call arguments")]
    Arg,

    /// An operand.
    #[error("statements cannot be used as operands")]
    Operand,

    /// A condition.
    #[error("statements cannot be used as conditions")]
    Condition,
}
