mod display;

/// An intermediate representation of a program.
pub struct Ir(pub Body);

/// A sequence of [`Instruction`]s in a program or function body.
pub struct Body(pub Box<[Instruction]>);

/// An executable instruction.
pub enum Instruction {
    /// Push a constant [`Value`] to the stack.
    Push(Value),

    /// Load a [`Value`] from a global variable and push it to the stack.
    PushGlobal(String),

    /// Load a [`Value`] from a local variable and push it to the stack.
    PushLocal(usize),

    /// Pop a [`Value`] from the stack and store it in a global variable.
    StoreGlobal(String),

    /// Pop a [`Value`] from the stack and store it in a local variable.
    StoreLocal(usize),

    /// Pop a [`Value`] from the stack and discard it.
    Pop,

    /// Pop a [`Value`] from the stack and print it.
    Print,

    /// Pop a [`Value`] from the stack, perform a unary operation on it, and
    /// push the result to the stack.
    Unary(UnOp),

    /// Pop two [`Value`]s from the stack, perform a binary operation on them,
    /// and push the result to the stack.
    Binary(BinOp),

    /// Halt execution.
    Halt,
}

/// A value with a dynamic type.
#[derive(Clone)]
pub enum Value {
    /// An empty value.
    Void,

    /// A number.
    Number(f64),
}

/// A unary operation.
#[derive(Clone, Copy)]
pub enum UnOp {
    /// A negation.
    Negate,
}

/// A binary operation.
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
