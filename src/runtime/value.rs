use std::{fmt, ops};

use crate::ast::Literal;

/// A value with a dynamic type.
#[derive(Clone)]
pub enum Value {
    /// A number value.
    Number(f64),
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
        }
    }
}

impl ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(rhs) => Self::Number(-rhs),
        }
    }
}

macro_rules! value_binop_impl {
    ($trait:path, $fn:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Self;

            fn $fn(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Self::Number(lhs), Self::Number(rhs)) => Self::Number(lhs $op rhs),
                }
            }
        }
    }
}

value_binop_impl!(ops::Add, add, +);
value_binop_impl!(ops::Sub, sub, -);
value_binop_impl!(ops::Mul, mul, *);
value_binop_impl!(ops::Div, div, /);
