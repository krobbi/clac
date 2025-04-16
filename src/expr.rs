use std::fmt;

use crate::bin_op::BinOp;

/// An expression.
pub enum Expr {
    /// A literal expression.
    Literal(f64),

    /// A unary negation expression.
    Negate(Box<Expr>),

    /// A binary expression.
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(value) => value.fmt(f),
            Self::Negate(expr) => write!(f, "-{expr}"),
            Self::Binary { lhs, op, rhs } => write!(f, "({lhs} {op} {rhs})"),
        }
    }
}
