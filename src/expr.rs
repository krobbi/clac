use crate::bin_op::BinOp;

/// An expression.
pub enum Expr {
    /// A literal expression.
    Literal(f64),

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
