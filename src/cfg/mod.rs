mod display;

use std::rc::Rc;

use crate::{ast::Literal, symbols::Symbol};

/// A control flow graph.
#[derive(Debug)]
pub struct Cfg {
    // NOTE: This should be changed to a hash map or a similar structure if the
    // basic blocks need to be rearranged (e.g. if CFG optimizations are added),
    // but a vector has a faster lookup time.
    basic_blocks: Vec<BasicBlock>,
}

impl Cfg {
    /// Creates a new `Cfg`.
    pub fn new() -> Self {
        let mut cfg = Self {
            basic_blocks: Vec::new(),
        };

        let main_label = cfg.insert_basic_block();
        debug_assert_eq!(
            main_label,
            Label::default(),
            "main basic block should have the default label"
        );

        cfg
    }

    /// Inserts a new [`BasicBlock`] into the `Cfg` and returns its [`Label`].
    pub fn insert_basic_block(&mut self) -> Label {
        self.basic_blocks.push(BasicBlock {
            instructions: Vec::new(),
            terminator: Terminator::Halt,
        });

        Label(self.basic_blocks.len() - 1)
    }

    /// Returns a reference to a [`BasicBlock`] from its [`Label`].
    pub fn basic_block(&self, label: Label) -> &BasicBlock {
        &self.basic_blocks[label.0]
    }

    /// Returns a mutable reference to a [`BasicBlock`] from its [`Label`].
    pub fn basic_block_mut(&mut self, label: Label) -> &mut BasicBlock {
        &mut self.basic_blocks[label.0]
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

/// A label for a [`BasicBlock`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Label(usize);

/// A basic block.
#[derive(Debug)]
pub struct BasicBlock {
    /// The [`Instruction`]s.
    pub instructions: Vec<Instruction>,

    /// The [`Terminator`].
    pub terminator: Terminator,
}

/// An instruction which can appear in the middle of a [`BasicBlock`].
#[derive(Debug)]
pub enum Instruction {
    /// Pushes a [`Literal`] value to the stack.
    PushLiteral(Literal),

    /// Pushes a [`Function`] value to the stack.
    PushFunction(Rc<Function>),

    /// Loads a value from a global variable and pushes it to the stack.
    PushGlobal(Symbol),

    /// Loads a value from a stack frame offset and pushes it to the stack.
    PushLocal(usize),

    /// Loads a value from an upvar stack offset and pushes it to the stack.
    PushUpvar(usize),

    /// Pops a number of values from the stack and discards them.
    Pop(usize),

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

    /// Pops a subtrahend number value from the stack, then a minuend number
    /// value. The subtrahend is subtracted from the minuend and the result is
    /// pushed to the stack.
    Subtract,

    /// Pops two number values from the stack, multiplies them, and pushes the
    /// result to the stack.
    Multiply,

    /// Pops a divisor number value from the stack, then a dividend number
    /// value. The dividend is divided by the divisor and the result is pushed
    /// to the stack.
    Divide,

    /// Pops an exponent number value from the stack, then a base number value.
    /// The base is raised to the power of the exponent and the result is pushed
    /// to the stack.
    Power,

    /// Pops two values from the stack, compares them as equal, and pushes the
    /// result to the stack.
    Equal,

    /// Pops two values from the stack, compares them as not equal, and pushes
    /// the result to the stack.
    NotEqual,

    /// Pops a right-hand side number value from the stack, then a left-hand
    /// side number value. The left-hand is compared as less than the right-hand
    /// and the result is pushed to the stack.
    Less,

    /// Pops a right-hand side number value from the stack, then a left-hand
    /// side number value. The left-hand is compared as less than or equal to
    /// the right-hand and the result is pushed to the stack.
    LessEqual,

    /// Pops a right-hand side number value from the stack, then a left-hand
    /// side number value. The left-hand is compared as greater than the
    /// right-hand and the result is pushed to the stack.
    Greater,

    /// Pops a right-hand side number value from the stack, then a left-hand
    /// side number value. The left-hand is compared as greater than or equal to
    /// the right-hand and the result is pushed to the stack.
    GreaterEqual,

    /// Pops a value from the stack and stores it in a local variable.
    StoreGlobal(Symbol),

    /// Pops a value from the stack and stores it at a stack frame offset.
    StoreLocal(usize),

    /// Pops a value from the stack and pushes it to the upvar stack.
    DefineUpvar,

    /// Pops a number of values from the upvar stack and discards them.
    PopUpvars(usize),

    /// Pops a [`Function`] value from the stack, converts it to a closure, and
    /// pushes the result to the stack.
    IntoClosure,
}

/// A [`BasicBlock`]'s terminator.
#[derive(Debug)]
pub enum Terminator {
    /// Halts execution.
    Halt,

    /// Unconditionally jumps to a [`Label`].
    Jump(Label),

    /// Pops a Boolean value from the stack and jumps to a [`Label`] if it is
    /// [`true`], or jumps to another [`Label`] if it is [`false`].
    Branch(Label, Label),

    /// Performs a call with an arity and returns to a [`Label`].
    Call(usize, Label),

    /// Pops a value from the top of the stack and returns it.
    Return,
}
