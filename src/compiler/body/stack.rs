use crate::decl_table::DeclId;

/// A stack of local variables and intermediate values.
#[derive(Default)]
pub struct Stack {
    /// The declared [`Elem`]s.
    elems: Vec<Elem>,

    /// The stack offsets to each scope.
    scope_offsets: Vec<usize>,
}

impl Stack {
    /// Creates a new `Stack`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of local variables and intermediate values on the
    /// `Stack`.
    pub fn len(&self) -> usize {
        self.elems.len()
    }

    /// Begins a scope.
    pub fn begin_scope(&mut self) {
        self.scope_offsets.push(self.len());
    }

    /// Ends a scope and returns the number of local variables that were
    /// declared in the scope.
    pub fn end_scope(&mut self) -> usize {
        let stack_offset = self
            .scope_offsets
            .pop()
            .expect("scope stack should not be empty");

        #[cfg(debug_assertions)]
        for elem in &self.elems[stack_offset..] {
            debug_assert!(
                matches!(elem, Elem::Local(_)),
                "dropped elements should only be local variables"
            );
        }

        let local_count = self.len() - stack_offset;
        self.elems.truncate(stack_offset);
        local_count
    }

    /// Declares a new local variable at the top of the `Stack`.
    pub fn declare_local(&mut self, id: DeclId) {
        self.elems.push(Elem::Local(id));
    }

    /// Declares a new intermediate value at the top of the `Stack`.
    pub fn declare_intermediate(&mut self) {
        self.elems.push(Elem::Intermediate);
    }

    /// Declares the removal of a number of intermediate values from the top of
    /// the `Stack`.
    pub fn drop_intermediates(&mut self, count: usize) {
        let stack_offset = self.len() - count;

        #[cfg(debug_assertions)]
        for elem in &self.elems[stack_offset..] {
            debug_assert!(
                matches!(elem, Elem::Intermediate),
                "dropped elements should only be intermediate values"
            );
        }

        self.elems.truncate(stack_offset);
    }

    /// Returns the stack offset of a local variable.
    pub fn local_offset(&self, id: DeclId) -> usize {
        for (offset, elem) in self.elems.iter().enumerate() {
            if let Elem::Local(declared_id) = elem
                && *declared_id == id
            {
                return offset;
            }
        }

        unreachable!("local variable should be declared");
    }
}

// A stack element.
enum Elem {
    /// A local variable.
    Local(DeclId),

    /// An intermediate value.
    Intermediate,
}
