use crate::decl_table::DeclId;

/// A stack of local variables and intermediate values.
#[derive(Default)]
pub struct Stack {
    /// The declared stack elements.
    elems: Vec<Elem>,

    /// The stack offsets to each scope.
    scope_offsets: Vec<usize>,
}

impl Stack {
    /// Creates a new `Stack`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new scope to the `Stack`.
    pub fn push_scope(&mut self) {
        self.scope_offsets.push(self.len());
    }

    /// Pops a scope from the `Stack` and returns the number of local
    /// variables that were declared in the popped scope.
    pub fn pop_scope(&mut self) -> usize {
        let scope_offset = self
            .scope_offsets
            .pop()
            .expect("scope stack should not be empty");

        #[cfg(debug_assertions)]
        for elem in &self.elems[scope_offset..] {
            debug_assert!(
                matches!(elem, Elem::Local(_)),
                "popped scope should not contain intermediate values"
            );
        }

        let local_count = self.len() - scope_offset;
        self.elems.truncate(scope_offset);
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

    /// Declares the removal of an intermediate value from the top of the
    /// `Stack`.
    pub fn declare_drop_intermediate(&mut self) {
        let dropped = self.elems.pop();

        debug_assert!(
            matches!(dropped, Some(Elem::Intermediate)),
            "there should be an intermediate value on top of the stack"
        );
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

    /// Returns the number of local variables and intermediate values on the
    /// `Stack`.
    pub fn len(&self) -> usize {
        self.elems.len()
    }
}

// A stack element.
enum Elem {
    /// A local variable.
    Local(DeclId),

    /// An intermediate value.
    Intermediate,
}
