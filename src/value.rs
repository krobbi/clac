use std::fmt;

/// A dynamically-typed value.
#[derive(Debug, Clone)]
pub enum Value {
    /// A value with a floating-point number type.
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
        }
    }
}
