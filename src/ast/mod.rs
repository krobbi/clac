mod display;

use crate::symbols::Symbol;

/// An abstract syntax tree.
#[derive(Debug)]
pub struct Ast(pub Box<[Expr]>);

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A [`Literal`].
    Literal(Literal),

    /// An identifier.
    Ident(Symbol),

    /// A parenthesized expression.
    Paren(Box<Self>),

    /// A tuple.
    Tuple(Box<[Self]>),

    /// A block.
    Block(Box<[Self]>),

    /// An assignment.
    Assign(Box<Self>, Box<Self>),

    /// A function.
    Function(Box<[Self]>, Box<Self>),

    /// A function call.
    Call(Box<Self>, Box<[Self]>),

    /// A unary operation.
    Unary(UnOp, Box<Self>),

    /// A binary operation.
    Binary(BinOp, Box<Self>, Box<Self>),

    /// A short-circuiting logical operation.
    Logic(LogicOp, Box<Self>, Box<Self>),

    /// A ternary conditional.
    Cond(Box<Self>, Box<Self>, Box<Self>),
}

/// A value which can be represented with a single
/// [`Token`][crate::tokens::Token].
#[derive(Clone, Debug)]
pub enum Literal {
    /// A number.
    Number(f64),

    /// A Boolean value.
    Bool(bool),
}

/// A unary operator.
#[derive(Clone, Copy, Debug)]
pub enum UnOp {
    /// A negation.
    Negate,

    /// A logical negation.
    Not,
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

    /// An exponentiation.
    Power,

    /// An equality comparison.
    Equal,

    /// An inequality comparison.
    NotEqual,

    /// A less than comparison.
    Less,

    /// A less than or equal to comparison.
    LessEqual,

    /// A greater than comparison.
    Greater,

    /// A greater than or equal to comparison.
    GreaterEqual,
}

/// A short-circuiting logical operator.
#[derive(Clone, Copy, Debug)]
pub enum LogicOp {
    /// A logical and.
    And,

    /// A logical or.
    Or,
}
