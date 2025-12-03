mod display;

/// An abstract syntax tree.
#[derive(Debug)]
pub struct Ast(pub Vec<Stmt>);

/// A statement.
#[derive(Debug)]
pub enum Stmt {
    /// An assignment.
    Assign(Box<Expr>, Box<Expr>),

    /// An expression statement.
    Expr(Box<Expr>),
}

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A [`Literal`].
    Literal(Literal),

    /// An identifier.
    Ident(String),

    /// A parenthesized expression.
    Paren(Box<Expr>),

    /// A tuple.
    Tuple(Vec<Expr>),

    /// A block.
    Block(Vec<Stmt>),

    /// A function.
    Function(Vec<Expr>, Box<Expr>),

    /// A function call.
    Call(Box<Expr>, Vec<Expr>),

    /// A unary operation.
    Unary(UnOp, Box<Expr>),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
}

/// A value that can be represented with a single token.
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

    /// A logical equality comparison.
    Equal,

    /// A logical inequality comparison.
    NotEqual,
}
