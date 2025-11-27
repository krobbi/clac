mod display;

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
        Self {
            blocks: vec![Block { exit: Exit::Halt }],
        }
    }
}

/// A label for a [`Block`].
#[derive(Clone, Copy, Default)]
pub struct Label(usize);

/// A basic block.
pub struct Block {
    /// The [`Exit`].
    pub exit: Exit,
}

/// A [`Block`]'s exit point.
pub enum Exit {
    /// Halts execution.
    Halt,
}
