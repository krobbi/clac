/// A stack of scoped local variable declarations.
pub struct Locals {
    /// The declared variable names in stack order.
    names: Vec<String>,

    /// The stack offsets to each scope.
    scope_offsets: Vec<usize>,
}

impl Locals {
    /// Creates new `Locals`.
    pub fn new() -> Self {
        let names = Vec::new();
        let scope_offsets = Vec::new();

        Self {
            names,
            scope_offsets,
        }
    }

    /// Pushes a new local scope.
    pub fn push_scope(&mut self) {
        self.scope_offsets.push(self.count());
    }

    /// Pops a local scope and returns the number of variables that were
    /// declared in the popped scope.
    pub fn pop_scope(&mut self) -> usize {
        let scope_offset = self
            .scope_offsets
            .pop()
            .expect("should not pop from an empty scope stack");

        let variable_count = self.count() - scope_offset;
        self.names.truncate(scope_offset);
        variable_count
    }

    /// Returns whether the current scope is the global scope.
    pub fn is_global_scope(&self) -> bool {
        self.scope_offsets.is_empty()
    }

    /// Returns the current number of local variable declarations, including
    /// shadowing and temporaries.
    pub fn count(&self) -> usize {
        self.names.len()
    }

    /// Returns the index of a local variable declaration. This function returns
    /// [`None`] if the name is not declared as a local variable.
    pub fn get(&self, name: &str) -> Option<usize> {
        for (index, declared_name) in self.names.iter().enumerate().rev() {
            if declared_name == name {
                return Some(index);
            }
        }

        None
    }

    /// Declares a variable in the current local scope. This function returns
    /// `true` if a new variable was declared and returns `false` if the
    /// variable was already declared.
    pub fn declare(&mut self, name: &str) -> bool {
        if self.top_contains(name) {
            false
        } else {
            self.names.push(name.to_owned());
            true
        }
    }

    /// Pushes a temporary local variable to the current scope.
    pub fn push_temp(&mut self) {
        self.names.push(String::new());
    }

    /// Pops a temporary local variable from the current scope.
    pub fn pop_temp(&mut self) {
        self.names.pop();
    }

    /// Returns `true` if the top local scope contains a variable.
    fn top_contains(&self, name: &str) -> bool {
        let scope_offset = self
            .scope_offsets
            .last()
            .copied()
            .expect("should not read from an empty scope stack");

        self.names[scope_offset..].contains(&name.to_owned())
    }
}
