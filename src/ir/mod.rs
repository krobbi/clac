#![expect(dead_code, reason = "functions are not yet implemented")]

use std::rc::Rc;

mod display;

/// An intermediate representation of a program.
pub struct Ir(pub Body);

/// A function.
pub struct Function(pub usize, pub Body);

/// A sequence of [`Instruction`]s in a program or function body.
pub struct Body(pub Box<[Instruction]>);

/// An executable instruction.
pub enum Instruction {
    /// Push a constant [`Value`] to the stack.
    Push(Value),

    /// Pop a [`Value`] from the stack and discard it.
    Drop,

    /// Pop a [`Value`] from the stack and print it.
    Print,

    /// Load a [`Value`] from a local variable and push it to the stack.
    LoadLocal(usize),

    /// Load a [`Value`] from a global variable and push it to the stack.
    LoadGlobal(String),

    /// Pop a [`Value`] from the stack and store it in a local variable.
    StoreLocal(usize),

    /// Pop a [`Value`] from the stack and store it in a global variable.
    StoreGlobal(String),

    /// Pop two [`Value`]s from the stack, perform a binary operation on them,
    /// and push the result to the stack.
    Binary(BinOp),

    /// Call a function with a number of argument [`Value`]s on the stack.
    Call(usize),

    /// Pop a [`Value`] from the stack, clear the current function's stack,
    /// return to the call site, and push the [`Value`] to the stack.
    Return,

    /// Halt execution.
    Halt,
}

/// A value with a dynamic type.
#[derive(Clone)]
pub enum Value {
    /// A number.
    Number(f64),

    /// A function.
    Function(Rc<Function>),
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
