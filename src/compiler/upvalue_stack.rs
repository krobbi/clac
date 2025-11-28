use crate::decl_table::DeclId;

/// A stack of upvalues.
#[derive(Default)]
pub struct UpvalueStack {
    /// The stack of [`DeclId`]s that are declared as upvalues.
    stack: Vec<DeclId>,

    /// The stack offsets to each scope.
    scope_offsets: Vec<usize>,
}

impl UpvalueStack {
    /// Creates a new `UpvalueStack`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Begins a scope.
    pub fn begin_scope(&mut self) {
        self.scope_offsets.push(self.stack.len());
    }

    /// Ends a scope and returns the number of upvalues that were declared in
    /// the scope.
    pub fn end_scope(&mut self) -> usize {
        let offset = self
            .scope_offsets
            .pop()
            .expect("scope stack should not be empty");

        let upvalue_count = self.stack.len() - offset;
        self.stack.truncate(offset);
        upvalue_count
    }

    /// Declares a new upvalue.
    pub fn declare(&mut self, id: DeclId) {
        debug_assert!(
            !self.scope_offsets.is_empty(),
            "scope stack should not be empty",
        );

        self.stack.push(id);
    }

    /// Returns the stack offset of an upvalue.
    pub fn offset(&self, id: DeclId) -> usize {
        self.stack
            .iter()
            .position(|i| *i == id)
            .expect("upvalue should exist")
    }
}
