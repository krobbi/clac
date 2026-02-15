use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use crate::{ast::Literal, cfg::Function};

use super::native::Native;

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

    /// A [`Native`].
    Native(Native),
}

impl Value {
    /// Returns [`true`] if the `Value`'s [`ValueType`] matches another
    /// `Value`'s [`ValueType`].
    pub fn matches_value_type(&self, other: &Self) -> bool {
        self.value_type() == other.value_type()
    }

    /// Returns the `Value`'s [`ValueType`].
    const fn value_type(&self) -> ValueType {
        match self {
            Self::Number(_) => ValueType::Number,
            Self::Bool(_) => ValueType::Bool,
            Self::Function(_) | Self::Closure(_) | Self::Native(_) => ValueType::Function,
        }
    }
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Number(value) => Self::Number(value),
            Literal::Bool(value) => Self::Bool(value),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(lhs), Self::Number(rhs)) => lhs == rhs,
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            (Self::Function(lhs), Self::Function(rhs)) => Rc::ptr_eq(lhs, rhs),
            (Self::Closure(lhs), Self::Closure(rhs)) => {
                if Rc::ptr_eq(lhs, rhs) {
                    return true;
                }

                if !Rc::ptr_eq(&lhs.function, &rhs.function) {
                    return false;
                }

                debug_assert_eq!(
                    lhs.upvars.len(),
                    rhs.upvars.len(),
                    "closures with the same function should have the same number of upvars"
                );

                for (lhs_upvar, rhs_upvar) in lhs.upvars.iter().zip(rhs.upvars.iter()) {
                    if lhs_upvar != rhs_upvar {
                        return false;
                    }
                }

                true
            }
            (Self::Native(lhs), Self::Native(rhs)) => lhs == rhs,
            (
                Self::Number(_)
                | Self::Bool(_)
                | Self::Function(_)
                | Self::Closure(_)
                | Self::Native(_),
                _,
            ) => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Number(lhs), Self::Number(rhs)) => lhs.partial_cmp(rhs),
            (lhs, rhs) => (lhs == rhs).then_some(Ordering::Equal),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => Display::fmt(value, f),
            Self::Bool(value) => Display::fmt(value, f),
            Self::Function(_) | Self::Closure(_) | Self::Native(_) => f.write_str("function"),
        }
    }
}

/// A [`Function`] with captured upvars.
pub struct Closure {
    /// The [`Function`].
    pub function: Rc<Function>,

    /// The upvars.
    pub upvars: Vec<Rc<Value>>,
}

/// A type of [`Value`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum ValueType {
    /// A number.
    Number,

    /// A Boolean value.
    Bool,

    /// A [`Function`], [`Closure`], or [`Native`].
    Function,
}
