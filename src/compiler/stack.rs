/// A stack of local variables and intermediate values.
pub struct Stack {
    /// The declared stack elements.
    elems: Vec<String>,

    /// The offsets to each scope.
    scope_offsets: Vec<usize>,
}

impl Stack {
    /// Creates a new `Stack`.
    pub fn new() -> Self {
        let elems = Vec::new();
        let scope_offsets = Vec::new();

        Self {
            elems,
            scope_offsets,
        }
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
        {
            for elem in &self.elems[scope_offset..] {
                debug_assert!(
                    !elem.is_empty(),
                    "popped scope should not contain intermediate values"
                );
            }
        }

        let local_count = self.len() - scope_offset;
        self.elems.truncate(scope_offset);
        local_count
    }

    /// Declares a new local variable at the top of the `Stack`.
    pub fn declare_local(&mut self, name: &str) {
        debug_assert!(
            !self.scope_offsets.is_empty(),
            "scope stack should not be empty"
        );

        self.elems.push(name.to_owned());
    }

    /// Declares a new intermediate value at the top of the `Stack`.
    pub fn declare_intermediate(&mut self) {
        self.elems.push(String::new());
    }

    /// Declares the removal of an intermediate value from the top of the
    /// `Stack`.
    pub fn declare_drop_intermediate(&mut self) {
        debug_assert!(
            matches!(self.elems.last(), Some(n) if n.is_empty()),
            "there should be an intermediate value on top of the stack"
        );

        self.elems.pop();
    }

    /// Returns the index of a local variable.
    pub fn local_index(&self, name: &str) -> usize {
        for (index, declared_name) in self.elems.iter().enumerate().rev() {
            if declared_name == name {
                return index;
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
