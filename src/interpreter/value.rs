use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use crate::{ast::Literal, cfg::Function};

/// A runtime value.
#[derive(Clone)]
pub enum Value {
    /// A number.
    Number(f64),

    /// A [`Function`].
    Function(Rc<Function>),

    /// A [`Closure`].
    Closure(Rc<Closure>),
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
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Function(_) | Self::Closure(_) => f.write_str("function"),
        }
    }
}
