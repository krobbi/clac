#![expect(dead_code, reason = "ASTs should be debug printed")]

/// An abstract syntax tree.
#[derive(Debug)]
pub struct Ast(pub Expr);

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A number.
    Number(f64),

    /// A parenthesized expression.
    Paren(Box<Expr>),
}
