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
}
