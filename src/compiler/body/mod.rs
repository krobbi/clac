mod stack;

use self::stack::Stack;

/// Context for compiling a program or function's body.
pub struct Body {
    /// The [`Stack`] for tracking the locations of local variables and
    /// intermediate values.
    pub stack: Stack,

    /// The shallowest call depth where an accessed upvalue was defined.
    pub upvalue_call_depth: usize,
}

impl Body {
    /// Creates a new `Body` from a call depth.
    pub fn new(call_depth: usize) -> Self {
        Self {
            stack: Stack::new(),
            upvalue_call_depth: call_depth,
        }
    }

    /// Declares the access of an upvalue declared at a call depth.
    pub fn access_upvalue(&mut self, call_depth: usize) {
        self.upvalue_call_depth = self.upvalue_call_depth.min(call_depth);
    }
}
