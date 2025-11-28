use std::fmt::{self, Display, Formatter};

use crate::ast::Literal;

/// A runtime value.
#[derive(Clone)]
pub enum Value {
    /// A number.
    Number(f64),
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
        }
    }
}
