#![expect(dead_code, reason = "HIR should be debug printed")]

/// A high level intermediate representation of a program.
#[derive(Debug)]
pub struct Hir(pub Vec<Stmt>);

/// A statement.
#[derive(Debug)]
pub enum Stmt {
    /// A print statement.
    Print(Box<Expr>),
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A number.
    Number(f64),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
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
