use std::collections::HashMap;

use crate::decl_table::{DeclId, DeclTable};

/// A scoped map of local variable names to [`DeclId`]s.
pub struct Locals<'a> {
    /// The [`DeclTable`].
    decls: &'a mut DeclTable,

    /// The stack of scopes.
    scopes: Vec<HashMap<String, DeclId>>,

    /// The stack of function scope depths.
    functions: Vec<usize>,
}

impl<'a> Locals<'a> {
    /// Creates new `Locals` from a [`DeclTable`].
    pub fn new(decls: &'a mut DeclTable) -> Self {
        Self {
            decls,
            scopes: Vec::new(),
            functions: Vec::new(),
        }
    }

    /// Begins a block.
    pub fn begin_block(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Ends a block.
    pub fn end_block(&mut self) {
        debug_assert!(!self.scopes.is_empty(), "scope stack should not be empty");
        self.scopes.pop();
    }

    /// Begins a function.
    pub fn begin_function(&mut self) {
        let depth = self.scopes.len();
        self.scopes.push(HashMap::new());
        self.functions.push(depth);
    }

    /// Ends a function.
    pub fn end_function(&mut self) {
        debug_assert!(
            !self.functions.is_empty(),
            "function stack should not be empty"
        );
        debug_assert!(!self.scopes.is_empty(), "scope stack should not be empty");
        self.functions.pop();
        self.scopes.pop();
    }

    /// Declares a local variable and returns its [`DeclId`].
    pub fn declare(&mut self, name: &str) -> DeclId {
        let scope = self
            .scopes
            .last_mut()
            .expect("scope stack should not be empty");
        debug_assert!(
            !scope.contains_key(name),
            "variable should not be already defined"
        );
        let id = self.decls.declare();
        scope.insert(name.to_owned(), id);
        id
    }

    /// Reads a local variable and returns its [`DeclId`]. This function returns
    /// [`None`] if the local variable is undefined.
    pub fn read(&mut self, name: &str) -> Option<DeclId> {
        for (depth, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(id) = scope.get(name).copied() {
                let function_depth = self.functions.last().copied().unwrap_or(0);

                if depth < function_depth {
                    self.decls.get_mut(id).is_upvalue = true;
                }

                return Some(id);
            }
        }

        None
    }

    /// Returns `true` if a local variable is declared in the innermost scope.
    pub fn contains_inner(&self, name: &str) -> bool {
        let scope = self.scopes.last().expect("scope stack should not be empty");
        scope.contains_key(name)
    }
}
