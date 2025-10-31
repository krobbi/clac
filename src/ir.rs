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

    /// Pop a [`Value`] from the stack and print it.
    Print,

    /// Halt execution.
    Halt,
}

/// A value with a dynamic type.
#[derive(Debug, Clone)]
pub enum Value {
    /// A number.
    Number(f64),
}
