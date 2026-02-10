use crate::{
    ast::{BinOp, Literal, UnOp},
    decl_table::DeclId,
    symbols::Symbol,
};

/// A high-level intermediate representation of a program.
pub struct Hir(pub Vec<Stmt>);

/// A statement.
pub enum Stmt {
    /// No operation.
    Nop,

    /// A block.
    Block(Vec<Self>),

    /// A global variable assignment.
    AssignGlobal(Symbol, Box<Expr>),

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
    Global(Symbol),

    /// A local variable.
    Local(DeclId),

    /// A block.
    Block(Vec<Stmt>, Box<Self>),

    /// A function.
    Function(Vec<DeclId>, Box<Self>),

    /// A function call.
    Call(Box<Self>, Vec<Self>),

    /// A unary operation.
    Unary(UnOp, Box<Self>),

    /// A binary operation.
    Binary(BinOp, Box<Self>, Box<Self>),

    /// A ternary conditional.
    Cond(Box<Self>, Box<Self>, Box<Self>),
}
