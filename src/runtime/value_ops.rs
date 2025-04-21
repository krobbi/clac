use std::ops;

use crate::ast::Value;

use super::runtime_error::RuntimeError;

impl ops::Neg for Value {
    type Output = Result<Self, RuntimeError>;

    fn neg(self) -> Self::Output {
        match self {
            Self::Void => Err(RuntimeError::VoidArgument),
            Self::Number(rhs) => Ok(Self::Number(-rhs)),
        }
    }
}

macro_rules! value_binop_impl {
    ($trait:path, $fn:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Result<Self, RuntimeError>;

            fn $fn(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Value::Void, _) | (_, Value::Void) => Err(RuntimeError::VoidArgument),
                    (Value::Number(lhs), Value::Number(rhs)) => Ok(Value::Number(lhs $op rhs)),
                }
            }
        }
    }
}

value_binop_impl!(ops::Add, add, +);
value_binop_impl!(ops::Sub, sub, -);
value_binop_impl!(ops::Mul, mul, *);
value_binop_impl!(ops::Div, div, /);
