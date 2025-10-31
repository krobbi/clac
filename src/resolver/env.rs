use std::collections::HashSet;

/// An environment of defined variables.
pub struct Env {
    /// The names of the defined global variables.
    globals: HashSet<String>,

    /// The stack of [`Scope`]s.
    scopes: Vec<Scope>,
}

impl Env {
    /// Creates a new environment.
    pub fn new() -> Self {
        let globals = HashSet::new();
        let scopes = Vec::new();
        Self { globals, scopes }
    }

    /// Returns whether the environment is in the global [`Scope`].
    pub fn is_global(&self) -> bool {
        self.scopes.is_empty()
    }

    /// Returns the number of defined local variables in the current `Scope`.
    pub fn local_count(&self) -> u8 {
        match self.scopes.last() {
            None => 0,
            Some(scope) => scope
                .locals
                .len()
                .try_into()
                .expect("more than `u8::MAX` local variables should not be defined"),
        }
    }

    /// Pushes a new current [`Scope`] to the environment.
    pub fn push_scope(&mut self) {
        let base = match self.scopes.last() {
            None => 0,
            Some(scope) => scope.base + scope.locals.len(),
        };

        let scope = Scope::new(base);
        self.scopes.push(scope);
    }

    /// Pops the current [`Scope`] from the environment.
    pub fn pop_scope(&mut self) {
        debug_assert!(
            !self.scopes.is_empty(),
            "should not pop from an empty scope stack"
        );

        self.scopes.pop();
    }

    /// Defines a new variable and returns its [`Location`]. This function
    /// returns [`None`] if the variable is already defined.
    pub fn define(&mut self, name: &str) -> Option<Location> {
        match self.scopes.last_mut() {
            None => self
                .globals
                .insert(name.to_owned())
                .then_some(Location::Global),

            Some(scope) => {
                let name = name.to_owned();

                if scope.locals.contains(&name) {
                    return None;
                }

                let index = scope.base + scope.locals.len();
                let index = index
                    .try_into()
                    .expect("more than `u8::MAX` local variables should not be defined");

                scope.locals.push(name);
                Some(Location::Local(index))
            }
        }
    }

    /// Finds the [`Location`] of a variable. This function returns [`None`] if
    /// the variable is undefined.
    pub fn find(&self, name: &str) -> Option<Location> {
        for scope in self.scopes.iter().rev() {
            for (index, local_name) in scope.locals.iter().enumerate() {
                if name == local_name {
                    let index = scope.base + index;
                    let index = index
                        .try_into()
                        .expect("more than `u8::MAX` local variables should not be defined");

                    return Some(Location::Local(index));
                }
            }
        }

        self.globals.contains(name).then_some(Location::Global)
    }
}

/// A location of a defined variable.
#[derive(Clone, Copy)]
pub enum Location {
    /// A global variable.
    Global,

    /// A local variable.
    Local(u8),
}

/// A local scope.
struct Scope {
    /// The number of local variables defined in outer `Scope`s.
    base: usize,

    /// The names of the defined local variables in definition order.
    locals: Vec<String>,
}

impl Scope {
    /// Creates a new `Scope`.
    fn new(base: usize) -> Self {
        let locals = Vec::new();
        Self { base, locals }
    }
}
