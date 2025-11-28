use crate::decl_table::DeclId;

/// A stack frame of local variables and temporary values.
pub struct LocalStack {
    /// The stack of [`Elem`]s.
    stack: Vec<Elem>,

    /// The stack offsets to each scope.
    scope_offsets: Vec<usize>,

    /// The shallowest call depth where an accessed upvalue was defined.
    upvalue_call_depth: usize,
}

impl LocalStack {
    /// Creates a new `LocalStack` from a call depth.
    pub fn new(call_depth: usize) -> Self {
        Self {
            stack: Vec::new(),
            scope_offsets: Vec::new(),
            upvalue_call_depth: call_depth,
        }
    }

    /// Returns the number of values in the `LocalStack`.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Returns the shallowest call depth where an accessed upvalue was defined.
    /// This function returns the `LocalStack`'s own call depth if no upvalues
    /// were accessed.
    pub fn upvalue_call_depth(&self) -> usize {
        self.upvalue_call_depth
    }

    /// Begins a block.
    pub fn begin_block(&mut self) {
        self.scope_offsets.push(self.len());
    }

    /// Ends a block and returns the number of local variables that were
    /// declared in the block.
    pub fn end_block(&mut self) -> usize {
        let offset = self
            .scope_offsets
            .pop()
            .expect("scope stack should not be empty");

        #[cfg(debug_assertions)]
        for elem in &self.stack[offset..] {
            debug_assert!(
                matches!(elem, Elem::Local(_)),
                "dropped elements should be local variables",
            );
        }

        let local_count = self.len() - offset;
        self.stack.truncate(offset);
        local_count
    }

    /// Declares a new local variable.
    pub fn declare(&mut self, id: DeclId) {
        self.stack.push(Elem::Local(id));
    }

    /// Returns the stack offset of a local variable.
    pub fn offset(&self, id: DeclId) -> usize {
        self.stack
            .iter()
            .position(|e| matches!(e, Elem::Local(i) if *i == id))
            .expect("local variable should exist")
    }

    /// Pushes a temporary value to the `LocalStack`.
    pub fn push_temp(&mut self) {
        self.stack.push(Elem::Temp);
    }

    /// Drops a number of temporary values from the top of the `LocalStack`.
    pub fn drop_temps(&mut self, count: usize) {
        let offset = self.len() - count;

        #[cfg(debug_assertions)]
        for elem in &self.stack[offset..] {
            debug_assert!(
                matches!(elem, Elem::Temp),
                "dropped elements should be temporary values"
            );
        }

        self.stack.truncate(offset);
    }

    /// Accesses an upvalue declared at a call depth.
    pub fn access_upvalue(&mut self, call_depth: usize) {
        self.upvalue_call_depth = self.upvalue_call_depth.min(call_depth);
    }
}

/// A stack element.
enum Elem {
    /// A local variable.
    Local(DeclId),

    /// A temporary value.
    Temp,
}
