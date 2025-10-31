#![expect(dead_code, reason = "HIR should be debug printed")]

/// A high level intermediate representation of a program.
#[derive(Debug)]
pub struct Hir(pub Vec<Stmt>);

/// A statement.
#[derive(Debug)]
pub enum Stmt {
    /// A global variable assignment.
    AssignGlobal(String, Box<Expr>),

    /// A local variable definition.
    DefineLocal(Box<Expr>),

    /// A print statement.
    Print(Box<Expr>),

    /// An expression statement.
    Expr(Box<Expr>),
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A number.
    Number(f64),

    /// A global variable.
    Global(String),

    /// A local variable.
    Local(u8),

    /// A block with a number of defined local variables.
    Block(u8, Vec<Stmt>),

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
