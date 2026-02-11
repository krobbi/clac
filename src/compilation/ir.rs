use crate::{
    ast::{BinOp, Literal, UnOp},
    symbols::Symbol,
};

use super::locals::Local;

/// An intermediate representation of a program.
#[derive(Debug)]
pub struct Ir(pub Box<[Stmt]>);

/// A statement.
#[derive(Debug)]
pub enum Stmt {
    /// A block `Stmt`.
    Block(Box<[Self]>),

    /// A global variable assignment.
    AssignGlobal(Symbol, Box<Expr>),

    /// A local variable definition.
    DefineLocal(Local, Box<Expr>),

    /// An implicit print.
    Print(Box<Expr>),

    /// An `Expr`.
    Expr(Box<Expr>),
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A [`Literal`].
    Literal(Literal),

    /// A global variable.
    Global(Symbol),

    /// A local variable.
    Local(Local),

    /// A block `Expr`.
    Block(Box<[Stmt]>, Box<Self>),

    /// A function.
    Function(Box<[Local]>, Box<Self>),

    /// A function call.
    Call(Box<Self>, Box<[Self]>),

    /// A unary operation.
    Unary(UnOp, Box<Self>),

    /// A binary operation.
    Binary(BinOp, Box<Self>, Box<Self>),

    /// A ternary conditional.
    Cond(Box<Self>, Box<Self>, Box<Self>),
}
