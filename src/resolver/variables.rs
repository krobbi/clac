use std::collections::HashSet;

/// A stack of variable [`Scope`]s.
pub struct ScopeStack {
    /// The global [`Scope`].
    global_scope: Scope,

    /// The stack of local [`Scope`]s.
    local_scopes: Vec<Scope>,
}

impl ScopeStack {
    /// Creates a new `ScopeStack`.
    pub fn new() -> Self {
        let global_scope = Scope::new();
        let local_scopes = Vec::new();

        Self {
            global_scope,
            local_scopes,
        }
    }

    /// Pushes a new innermost local [`Scope`] to the `ScopeStack`.
    pub fn push_scope(&mut self) {
        self.local_scopes.push(Scope::new());
    }

    /// Pops the innermost local [`Scope`] from the `ScopeStack`.
    pub fn pop_scope(&mut self) {
        debug_assert!(
            !self.local_scopes.is_empty(),
            "scope stack should not be empty"
        );

        self.local_scopes.pop();
    }

    /// Defines a new variable in the innermost [`Scope`].
    pub fn define_variable(&mut self, name: &str) {
        let scope = match self.local_scopes.last_mut() {
            None => &mut self.global_scope,
            Some(scope) => scope,
        };

        scope.define(name);
    }

    /// Returns `true` if a variable is defined in the innermost [`Scope`].
    pub fn has_inner_variable(&self, name: &str) -> bool {
        let scope = match self.local_scopes.last() {
            None => &self.global_scope,
            Some(scope) => scope,
        };

        scope.has_variable(name)
    }

    /// Returns the [`ScopeKind`] where a variable is defined. This function
    /// returns [`None`] if the variable is undefined.
    pub fn resolve_variable(&self, name: &str) -> Option<ScopeKind> {
        for scope in self.local_scopes.iter().rev() {
            if scope.has_variable(name) {
                return Some(ScopeKind::Local);
            }
        }

        self.global_scope
            .has_variable(name)
            .then_some(ScopeKind::Global)
    }
}

/// A kind of `Scope` where a variable may defined.
#[derive(Clone, Copy)]
pub enum ScopeKind {
    /// At the top level of the program.
    Global,

    /// Inside a block.
    Local,
}

/// A scope of defined variables.
struct Scope {
    /// The set of variable names defined in the `Scope`.
    variables: HashSet<String>,
}

impl Scope {
    /// Creates a new `Scope`.
    fn new() -> Self {
        let variables = HashSet::new();
        Self { variables }
    }

    /// Defines a new variable in the `Scope`.
    fn define(&mut self, name: &str) {
        let is_new = self.variables.insert(name.to_owned());
        debug_assert!(is_new, "variables should not be redefined");
    }

    /// Returns `true` if a variable is defined in the `Scope`.
    fn has_variable(&self, name: &str) -> bool {
        self.variables.contains(name)
    }
}
