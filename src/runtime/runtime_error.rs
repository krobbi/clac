use std::{error, fmt};

/// An error encountered at runtime.
#[derive(Debug)]
pub enum RuntimeError {
    /// Void was used as a value.
    VoidValue,

    /// An undefined variable was evaluated.
    UndefinedVariable(String),

    /// A non-variable assignment target was assigned to.
    NonVariableAssignment,

    /// A function was defined with a non-identifier name.
    NonIdentFunctionName,

    /// A function was defined with a non-identifier parameter name.
    NonIdentParamName,

    /// A function was defined with a duplicate parameter name.
    DuplicateParamName,

    /// A non-function callee was called.
    NonFunctionCall,

    /// An incorrect number of arguments were passed to a function.
    IncorrectArgCount,

    /// Incorrect types were passed as arguments to an operator or a function.
    IncorrectArgTypes,
}

impl error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VoidValue => f.write_str("cannot use void as a value"),
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
            Self::NonVariableAssignment => f.write_str("cannot assign to a non-variable"),
            Self::NonIdentFunctionName => f.write_str("function names must be identifiers"),
            Self::NonIdentParamName => f.write_str("function parameter names must be identifiers"),
            Self::DuplicateParamName => {
                f.write_str("functions cannot have duplicate parameter names")
            }
            Self::NonFunctionCall => f.write_str("cannot call a non-function"),
            Self::IncorrectArgCount => f.write_str("incorrect argument count for function"),
            Self::IncorrectArgTypes => f.write_str("incorrect argument types for operation"),
        }
    }
}
