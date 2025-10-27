#![expect(dead_code, reason = "ASTs should be debug printed")]

/// An abstract syntax tree.
#[derive(Debug)]
pub struct Ast(pub Vec<Expr>);

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A number.
    Number(f64),

    /// A parenthesized expression.
    Paren(Box<Expr>),

    /// A unary operation.
    Unary(UnOp, Box<Expr>),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
}

/// A unary operator.
#[derive(Clone, Copy, Debug)]
pub enum UnOp {
    /// A negation.
    Negate,
}

/// A binary operator.
#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    /// An addition.
    Add,

    /// A subtraction.
    Subtract,

    /// A multiplication.
    Multiply,

    /// A division.
    Divide,
}
