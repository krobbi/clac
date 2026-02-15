mod display;

use std::rc::Rc;

use crate::{ast::Literal, symbols::Symbol};

/// A control flow graph.
#[derive(Debug)]
pub struct Cfg {
    // NOTE: Any labels in the CFG will break if these blocks are rearranged
    // (e.g. if CFG optimizations are added). This could be changed to a map,
    // but a vector should have a faster lookup time.
    /// The [`Block`]s.
    blocks: Vec<Block>,
}

impl Cfg {
    /// Creates a new `Cfg`.
    pub fn new() -> Self {
        let mut cfg = Self { blocks: Vec::new() };
        cfg.insert_block();
        cfg
    }

    /// Inserts a new [`Block`] into the `Cfg` and returns its [`Label`].
    pub fn insert_block(&mut self) -> Label {
        let index = self.blocks.len();
        self.blocks.push(Block::default());
        Label(index)
    }

    /// Returns a reference to a [`Block`] from its [`Label`].
    pub fn block(&self, label: Label) -> &Block {
        &self.blocks[label.0]
    }

    /// Returns a mutable reference to a [`Block`] from its [`Label`].
    pub fn block_mut(&mut self, label: Label) -> &mut Block {
        &mut self.blocks[label.0]
    }
}

/// A function.
#[derive(Debug)]
pub struct Function {
    /// The [`Cfg`].
    pub cfg: Cfg,

    /// The number of parameters.
    pub arity: usize,
}

/// A label for a [`Block`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Label(usize);

/// A basic block.
#[derive(Debug, Default)]
pub struct Block {
    /// The [`Instruction`]s.
    pub instructions: Vec<Instruction>,

    /// The [`Exit`].
    pub exit: Exit,
}

/// An instruction that may appear in the middle of a [`Block`].
#[derive(Debug)]
pub enum Instruction {
    /// Pushes a [`Literal`] value to the stack.
    PushLiteral(Literal),

    /// Pushes a [`Function`] value to the stack.
    PushFunction(Rc<Function>),

    /// Drops a number of values from the top of the stack.
    Drop(usize),

    /// Pops a value from the stack and prints it.
    Print,

    /// Pops a number value from the stack, negates it, and pushes the result to
    /// the stack.
    Negate,

    /// Pops a Boolean value from the stack, logically negates it, and pushes
    /// the result to the stack.
    Not,

    /// Pops two number values from the stack, adds them, and pushes the result
    /// to the stack.
    Add,

    /// Pops a right number value from the stack, then a left number value. The
    /// right number is subtracted from the left number and the result is pushed
    /// to the stack.
    Subtract,

    /// Pops two number values from the stack, multiplies them, and pushes the
    /// result to the stack.
    Multiply,

    /// Pops a right number value from the stack, then a left number value. The
    /// left number is divided by the right number and the result is pushed to
    /// the stack.
    Divide,

    /// Pops a right number value from the stack, then a left number value. The
    /// left number is raised to the power of the right number and the result is
    /// pushed to the stack.
    Power,

    /// Pops two values from the stack, compares them as equal, and pushes the
    /// result to the stack.
    Equal,

    /// Pops two values from the stack, compares them as not equal, and pushes
    /// the result to the stack.
    NotEqual,

    /// Pops a right number value from the stack, then a left number value. The
    /// left number is compared as less than the right number and the result is
    /// pushed to the stack.
    Less,

    /// Pops a right number value from the stack, then a left number value. The
    /// left number is compared as less than or equal to the right number and
    /// the result is pushed to the stack.
    LessEqual,

    /// Pops a right number value from the stack, then a left number value. The
    /// left number is compared as greater than the right number and the result
    /// is pushed to the stack.
    Greater,

    /// Pops a right number value from the stack, then a left number value. The
    /// left number is compared as greater than or equal to the right number and
    /// the result is pushed to the stack.
    GreaterEqual,

    /// Loads a value from a local variable and pushes it to the stack.
    LoadLocal(usize),

    /// Pops a value from the stack and stores it in a local variable.
    StoreLocal(usize),

    /// Loads a value from a global variable and pushes it to the stack.
    LoadGlobal(Symbol),

    /// Pops a value from the stack and stores it in a global variable.
    StoreGlobal(Symbol),

    /// Pops a value from the stack and pushes it to the upvalue stack.
    DefineUpvalue,

    /// Loads an upvalue and pushes it to the stack.
    LoadUpvalue(usize),

    /// Drops a number of upvalues from the top of the upvalue stack.
    DropUpvalues(usize),

    /// Pops a function value from the stack, converts it to a closure, and
    /// pushes the result to the stack.
    IntoClosure,
}

/// A [`Block`]'s exit point.
#[derive(Debug, Default)]
pub enum Exit {
    /// Halts execution.
    #[default]
    Halt,

    /// Jumps to a [`Label`].
    Jump(Label),

    /// Pops a Boolean value from the stack and jumps to a [`Label`] if it is
    /// `true`, or jumps to another [`Label`] if it is `false`.
    Branch(Label, Label),

    /// Performs a call with an arity and returns to a [`Label`].
    Call(usize, Label),

    /// Pops a value from the top of the stack and returns it.
    Return,
}
