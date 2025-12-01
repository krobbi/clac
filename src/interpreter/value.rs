use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use crate::{ast::Literal, cfg::Function};

use super::InterpretError;

/// A runtime value.
#[derive(Clone)]
pub enum Value {
    /// A number.
    Number(f64),

    /// A Boolean value.
    Bool(bool),

    /// A [`Function`].
    Function(Rc<Function>),

    /// A [`Closure`].
    Closure(Rc<Closure>),

    /// A native function.
    Native(fn(&[Value]) -> Result<Value, InterpretError>),
}

/// A [`Function`] with captured upvalues.
pub struct Closure {
    /// The [`Function`].
    pub function: Rc<Function>,

    /// The upvalues.
    pub upvalues: Vec<Rc<Value>>,
}

impl From<&Literal> for Value {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Number(value) => Self::Number(*value),
            Literal::Bool(value) => Self::Bool(*value),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Bool(value) => value.fmt(f),
            Self::Function(_) | Self::Closure(_) | Self::Native(_) => f.write_str("function"),
        }
    }
}
