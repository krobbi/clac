mod display;

/// An abstract syntax tree.
pub struct Ast(pub Vec<Expr>);

/// An expression.
pub enum Expr {
    /// A number.
    Number(f64),

    /// An identifier.
    Ident(String),

    /// A parenthesized expression.
    Paren(Box<Expr>),

    /// A function call.
    Call(Box<Expr>, Vec<Expr>),

    /// A unary operation.
    Unary(UnOp, Box<Expr>),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
}

/// A unary operator.
#[derive(Clone, Copy)]
pub enum UnOp {
    /// A negation.
    Negate,
}

/// A binary operator.
#[derive(Clone, Copy)]
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
