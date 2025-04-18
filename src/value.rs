use std::fmt;

/// A value with a dynamic type.
#[derive(Debug, Clone)]
pub enum Value {
    /// The void type. Holds no value and cannot be used as an argument.
    Void,

    /// A value with a floating-point number type.
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "Void"),
            Self::Number(value) => value.fmt(f),
        }
    }
}
