use std::collections::{HashMap, HashSet};

use crate::{
    locals::{Local, LocalTable},
    symbols::Symbol,
};

/// A variable's storage kind.
#[derive(Clone, Copy)]
pub enum Variable {
    /// A global variable.
    Global,

    /// A local variable or upvar.
    Local(Local),
}

/// A stack of variable scopes.
pub struct ScopeStack<'loc> {
    /// The [`LocalTable`].
    locals: &'loc mut LocalTable,

    /// The current function depth.
    function_depth: usize,

    /// The set of declared global variable [`Symbol`]s.
    global_symbols: HashSet<Symbol>,

    /// The stack of local scopes mapping [`Symbol`]s to [`Local`]s.
    local_scopes: Vec<HashMap<Symbol, Local>>,
}

impl<'loc> ScopeStack<'loc> {
    /// Creates a new `ScopeStack` from a [`LocalTable`].
    pub fn new(locals: &'loc mut LocalTable) -> Self {
        Self {
            locals,
            function_depth: 0,
            global_symbols: HashSet::new(),
            local_scopes: Vec::new(),
        }
    }

    /// Returns [`true`] if the current scope is the global scope.
    pub const fn is_global_scope(&self) -> bool {
        self.local_scopes.is_empty()
    }

    /// Returns a [`Variable`] from its [`Symbol`]. This function returns
    /// [`None`] if the [`Symbol`] is not declared in any accessible scope.
    pub fn variable(&mut self, symbol: Symbol) -> Option<Variable> {
        for local_scope in self.local_scopes.iter().rev() {
            if let Some(local) = local_scope.get(&symbol).copied() {
                let local_data = self.locals.data_mut(local);

                debug_assert!(
                    local_data.function_depth <= self.function_depth,
                    "local variables from inner functions should not be accessed"
                );

                // If a local variable is accessed from outside the function
                // where it is declared, then it may need to be an upvar.
                if local_data.function_depth < self.function_depth {
                    local_data.is_upvar = true;
                }

                return Some(Variable::Local(local));
            }
        }

        self.global_symbols
            .contains(&symbol)
            .then_some(Variable::Global)
    }

    /// Pushes a new function scope to the `ScopeStack`.
    pub fn push_function_scope(&mut self) {
        self.function_depth += 1;
        self.push_block_scope();
    }

    /// Pops the current function scope from the `ScopeStack`.
    pub fn pop_function_scope(&mut self) {
        debug_assert!(self.function_depth > 0, "there should be a function scope");
        self.pop_block_scope();
        self.function_depth -= 1;
    }

    /// Pushes a new function parameter scope to the `ScopeStack`.
    pub fn push_param_scope(&mut self) {
        debug_assert!(self.function_depth > 0, "there should be a function scope");
        self.push_block_scope();
    }

    /// Pops the current function parameter scope from the `ScopeStack`.
    pub fn pop_param_scope(&mut self) {
        debug_assert!(self.function_depth > 0, "there should be a function scope");
        self.pop_block_scope();
    }

    /// Pushes a new block scope to the `ScopeStack`.
    pub fn push_block_scope(&mut self) {
        self.local_scopes.push(HashMap::new());
    }

    /// Pops the current block scope from the `ScopeStack`.
    pub fn pop_block_scope(&mut self) {
        debug_assert!(
            !self.local_scopes.is_empty(),
            "there should be a local scope"
        );

        self.local_scopes.truncate(self.local_scopes.len() - 1);
    }

    /// Declares a new [`Variable`] in the current scope from its [`Symbol`].
    /// This function returns [`None`] if the [`Symbol`] is already declared in
    /// the current scope.
    pub fn declare_variable(&mut self, symbol: Symbol) -> Option<Variable> {
        if let Some(local_scope) = self.local_scopes.last_mut() {
            if local_scope.contains_key(&symbol) {
                return None;
            }

            let local = self.locals.declare_local(self.function_depth);
            local_scope.insert(symbol, local);
            Some(Variable::Local(local))
        } else {
            self.global_symbols
                .insert(symbol)
                .then_some(Variable::Global)
        }
    }
}
