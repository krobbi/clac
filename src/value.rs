use std::fmt;

/// A value with a dynamic type.
#[derive(Debug, Clone)]
pub enum Value {
    /// A value returned by expressions to represent returning no value.
    Void,

    /// A floating-point number value.
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, ""),
            Self::Number(value) => value.fmt(f),
        }
    }
}
