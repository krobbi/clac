use std::collections::HashSet;

/// An environment of defined variables.
pub struct Env {
    /// The names of the defined global variables.
    globals: HashSet<String>,
}

impl Env {
    /// Creates a new environment.
    pub fn new() -> Self {
        let globals = HashSet::new();
        Self { globals }
    }

    /// Defines a new variable and returns its [`Location`]. This function
    /// returns [`None`] if the variable is already defined.
    pub fn define(&mut self, name: &str) -> Option<Location> {
        self.globals
            .insert(name.to_owned())
            .then_some(Location::Global)
    }

    /// Finds the [`Location`] of a variable. This function returns [`None`] if
    /// the variable is undefined.
    pub fn find(&self, name: &str) -> Option<Location> {
        self.globals.contains(name).then_some(Location::Global)
    }
}

/// A location of a defined variable.
#[derive(Clone, Copy)]
pub enum Location {
    /// A global variable.
    Global,
}
