#![expect(dead_code, reason = "function values are not yet fully implemented")]

use std::fmt::{self, Display, Formatter};

use crate::ast::Literal;

use super::Label;

/// A runtime value.
#[derive(Clone)]
pub enum Value {
    /// A number.
    Number(f64),

    /// A function.
    Function(Box<Function>),
}

/// A function.
#[derive(Clone)]
pub struct Function {
    /// The [`Label`].
    pub label: Label,

    /// The number of parameters.
    pub arity: usize,
}

impl From<&Literal> for Value {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Number(value) => Self::Number(*value),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Function(_) => f.write_str("function"),
        }
    }
}
