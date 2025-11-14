use std::{collections::HashMap, rc::Rc};

use crate::{decl_table::DeclId, interpreter::InterpretError};

/// An intermediate representation of a program.
pub struct Ir(pub Body);

/// A function with a number of parameters and a [`Body`].
pub struct Function {
    /// The number of arguments expected by the `Function`.
    pub arity: usize,

    /// The `Function` [`Body`].
    pub body: Body,
}

/// A [`Function`] with a captured environment of upvalues.
pub struct Closure {
    /// The [`Function`].
    pub function: Rc<Function>,

    /// The environment of upvalues.
    pub upvalues: HashMap<DeclId, Rc<Value>>,
}

/// A sequence of [`Instruction`]s in a program or [`Function`] body.
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

    /// Pop a function [`Value`] from the stack, convert it to a closure, and
    /// push the result to the stack.
    IntoClosure,

    /// Call a function with a number of argument [`Value`]s on the stack.
    Call(usize),
}

/// A value with a dynamic type.
#[derive(Clone)]
pub enum Value {
    /// A number.
    Number(f64),

    /// A function.
    Function(Rc<Function>),

    /// A closure.
    Closure(Rc<Closure>),

    /// A native function.
    Native(fn(&[Value]) -> Result<Value, InterpretError>),
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
