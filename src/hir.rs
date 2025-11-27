use crate::{
    ast::{BinOp, Literal},
    decl_table::DeclId,
};

/// A high-level intermediate representation of a program.
pub struct Hir(pub Vec<Stmt>);

/// A statement.
pub enum Stmt {
    /// No operation.
    Nop,

    /// A block.
    Block(Vec<Stmt>),

    /// A global variable assignment.
    AssignGlobal(String, Box<Expr>),

    /// A local variable definition.
    DefineLocal(DeclId, Box<Expr>),

    /// A print statement.
    Print(Box<Expr>),

    /// An expression statement.
    Expr(Box<Expr>),
}

/// An expression.
pub enum Expr {
    /// A [`Literal`].
    Literal(Literal),

    /// A global variable.
    Global(String),

    /// A local variable.
    Local(DeclId),

    /// A block.
    Block(Vec<Stmt>, Box<Expr>),

    /// A function.
    Function(Vec<DeclId>, Box<Expr>),

    /// A function call.
    Call(Box<Expr>, Vec<Expr>),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
}
