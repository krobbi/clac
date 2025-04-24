use super::{BinOp, Literal};

/// An expression.
#[derive(Clone)]
pub enum Expr {
    /// A literal value expression.
    Literal(Literal),

    /// An identifier expression.
    Ident(String),

    /// A block expression.
    Block(Vec<Expr>),

    /// A call expression.
    Call { callee: Box<Expr>, args: Vec<Expr> },

    /// A unary negation expression.
    Negate(Box<Expr>),

    /// A binary expression.
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}
