use std::collections::{HashMap, HashSet};

use crate::ir::Value;

/// A map of global variables.
pub struct Globals {
    /// The map of global variable names to [`Value`]s.
    values: HashMap<String, Value>,
}

impl Globals {
    /// Creates new `Globals`.
    pub fn new() -> Self {
        let values = HashMap::new();
        Self { values }
    }

    /// Creates a new [`HashSet`] of all defined global variable names.
    pub fn names(&self) -> HashSet<String> {
        self.values.keys().cloned().collect()
    }

    /// Returns a reference to a global variable's [`Value`].
    pub fn get(&self, name: &str) -> &Value {
        &self.values[name]
    }

    /// Sets a global variable's [`Value`].
    pub fn set(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }
}
