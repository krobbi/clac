#![expect(dead_code, reason = "IR code should be debug printed")]

/// An intermediate representation of a program.
#[derive(Debug)]
pub struct Ir(pub Body);

/// A sequence of [`Instruction`]s in a program or function body.
#[derive(Debug)]
pub struct Body(pub Box<[Instruction]>);

/// An executable instruction.
#[derive(Debug)]
pub enum Instruction {
    /// Push a constant [`Value`] to the stack.
    PushValue(Value),

    /// Pop a [`Value`] from the stack, perform a unary operation on it, and
    /// push the result to the stack.
    Unary(UnOp),

    /// Pop two [`Value`]s from the stack, perform a binary operation on them,
    /// and push the result to the stack.
    Binary(BinOp),

    /// Pop a [`Value`] from the stack and print it.
    Print,

    /// Halt execution.
    Halt,
}

/// A value with a dynamic type.
#[derive(Clone, Debug)]
pub enum Value {
    /// A number.
    Number(f64),
}

/// A unary operation.
#[derive(Clone, Copy, Debug)]
pub enum UnOp {
    /// A negation.
    Negate,
}

/// A binary operation.
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
