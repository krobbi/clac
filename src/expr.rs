use crate::{bin_op::BinOp, value::Value};

/// An expression.
pub enum Expr {
    /// A literal value expression.
    Literal(Value),

    /// An identifier expression.
    Ident(String),

    /// A unary negation expression.
    Negate(Box<Expr>),

    /// A binary expression.
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}
