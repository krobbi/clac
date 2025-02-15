use std::fmt;

/// A mathematical expression.
pub enum Expr {
    /// A number.
    Number(f64),

    /// A parenthesized expression.
    Paren(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Paren(expr) => write!(f, "({expr})"),
        }
    }
}
