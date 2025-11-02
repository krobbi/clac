#![expect(dead_code, reason = "HIR fields should be debug printed")]

/// A high-level intermediate representation of a program.
#[derive(Debug)]
pub struct Hir(pub Vec<Stmt>);

/// A statement.
#[derive(Debug)]
pub enum Stmt {
    /// A print statement.
    Print(Expr),
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A number.
    Number(f64),
}
