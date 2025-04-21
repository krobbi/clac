use std::{fmt, ops};

use crate::ast::Literal;

use super::{EvalResult, runtime_error::RuntimeError};

/// A value with a dynamic type.
#[derive(Clone)]
pub enum Value {
    /// A number value.
    Number(f64),

    /// A built-in function.
    Builtin(fn(&[Value]) -> EvalResult),
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Number(value) => Value::Number(value),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Builtin(_) => f.write_str("function"),
        }
    }
}

impl ops::Neg for Value {
    type Output = Result<Self, RuntimeError>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(rhs) => Ok(Self::Number(-rhs)),
            Self::Builtin(_) => Err(RuntimeError::IncorrectArgTypes),
        }
    }
}

macro_rules! value_binop_impl {
    ($trait:path, $fn:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Result<Self, RuntimeError>;

            fn $fn(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Self::Number(lhs), Self::Number(rhs)) => Ok(Self::Number(lhs $op rhs)),
                    (_, _) => Err(RuntimeError::IncorrectArgTypes),
                }
            }
        }
    }
}

value_binop_impl!(ops::Add, add, +);
value_binop_impl!(ops::Sub, sub, -);
value_binop_impl!(ops::Mul, mul, *);
value_binop_impl!(ops::Div, div, /);
