/// A high-level intermediate representation of a program.
pub struct Hir(pub Vec<Stmt>);

/// A statement.
pub enum Stmt {
    /// No operation.
    Nop,

    /// A block.
    Block(Vec<Stmt>),

    /// A local variable definition.
    DefineLocal(String, Box<Expr>),

    /// A global variable assignment.
    AssignGlobal(String, Box<Expr>),

    /// A print statement.
    Print(Box<Expr>),

    /// An expression statement.
    Expr(Box<Expr>),
}

/// An expression.
pub enum Expr {
    /// A number.
    Number(f64),

    /// A local variable.
    Local(String),

    /// A global variable.
    Global(String),

    /// A block.
    Block(Vec<Stmt>, Box<Expr>),

    /// A function.
    Function(Vec<String>, Box<Expr>),

    /// A function call.
    Call(Box<Expr>, Vec<Expr>),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),
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
