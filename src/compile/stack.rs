use crate::locals::Local;

/// A stack frame.
#[derive(Default)]
pub struct StackFrame {
    /// The stack of [`Elem`]s.
    elems: Vec<Elem>,

    /// The stack offsets to each scope.
    scope_offsets: Vec<usize>,
}

impl StackFrame {
    /// Creates a new `StackFrame`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements in the `StackFrame`.
    pub const fn len(&self) -> usize {
        self.elems.len()
    }

    /// Returns a local variable's stack frame offset from its [`Local`].
    pub fn local_offset(&self, local: Local) -> usize {
        self.elems
            .iter()
            .position(|e| matches!(e, Elem::Local(l) if *l == local))
            .expect("local variable should exist")
    }

    /// Pushes a new local scope to the `StackFrame`.
    pub fn push_scope(&mut self) {
        self.scope_offsets.push(self.len());
    }

    /// Pops the current local scope from the `StackFrame` and returns the
    /// number of local variables that were declared in the local scope.
    pub fn pop_scope(&mut self) -> usize {
        let offset = self
            .scope_offsets
            .pop()
            .expect("there should be a local scope");

        #[cfg(debug_assertions)]
        for elem in &self.elems[offset..] {
            debug_assert!(
                matches!(elem, Elem::Local(_)),
                "popped elements should all be local variables"
            );
        }

        let local_count = self.len() - offset;
        self.elems.truncate(offset);
        local_count
    }

    /// Marks a local variable being pushed to the `StackFrame`.
    pub fn push_local(&mut self, local: Local) {
        debug_assert!(
            !self.scope_offsets.is_empty(),
            "there should be a local scope"
        );

        self.elems.push(Elem::Local(local));
    }

    /// Marks a callee being pushed to the `StackFrame`.
    pub fn push_callee(&mut self, local: Local) {
        debug_assert!(self.elems.is_empty(), "stack frame should be empty");
        debug_assert!(
            self.scope_offsets.is_empty(),
            "there should not be a local scope"
        );

        self.elems.push(Elem::Local(local));
    }

    /// Marks a function parameter being pushed to the `StackFrame`.
    pub fn push_param(&mut self, local: Local) {
        debug_assert!(!self.elems.is_empty(), "stack frame should not be empty");
        debug_assert!(
            self.scope_offsets.is_empty(),
            "there should not be a local scope"
        );

        self.elems.push(Elem::Local(local));
    }

    /// Marks a temporary value being pushed to the `StackFrame`.
    pub fn push_temp(&mut self) {
        self.elems.push(Elem::Temp);
    }

    /// Marks a number of temporary values being popped from the `StackFrame`.
    pub fn pop_temps(&mut self, count: usize) {
        let offset = self.len() - count;

        #[cfg(debug_assertions)]
        for elem in &self.elems[offset..] {
            debug_assert!(
                matches!(elem, Elem::Temp),
                "popped elements should all be temporary values"
            );
        }

        self.elems.truncate(offset);
    }
}

/// A stack element.
#[derive(Clone, Copy)]
enum Elem {
    /// A local variable.
    Local(Local),

    /// A temporary value.
    Temp,
}
