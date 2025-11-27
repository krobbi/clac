mod display;

use crate::ast::Literal;

/// A control flow graph.
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
        self.blocks.push(Block {
            instructions: Vec::new(),
            exit: Exit::Halt,
        });

        Label(index)
    }

    /// Returns a mutable reference to a [`Block`] from its [`Label`].
    pub fn block_mut(&mut self, label: Label) -> &mut Block {
        &mut self.blocks[label.0]
    }
}

/// A label for a [`Block`].
#[derive(Clone, Copy, Default)]
pub struct Label(usize);

/// A basic block.
pub struct Block {
    /// The [`Instruction`]s.
    pub instructions: Vec<Instruction>,

    /// The [`Exit`].
    pub exit: Exit,
}

/// An instruction that may appear in the middle of a [`Block`].
pub enum Instruction {
    /// Pushes a [`Literal`] value to the stack.
    PushLiteral(Literal),

    /// Pops a value from the stack and discards it.
    Drop,

    /// Pops a value from the stack and prints it.
    Print,

    /// Pops a value from the stack and stores it in a global variable.
    StoreGlobal(String),
}

/// A [`Block`]'s exit point.
pub enum Exit {
    /// Halts execution.
    Halt,

    /// Pops a value from the top of the stack and returns it.
    Return,
}
