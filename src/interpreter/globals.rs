use std::collections::HashMap;

use crate::symbols::Symbol;

use super::value::Value;

/// A map of global variables.
#[derive(Default)]
pub struct Globals {
    /// The map of [`Symbol`]s to [`Value`]s.
    values: HashMap<Symbol, Value>,
}

impl Globals {
    /// Creates new `Globals`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns [`true`] if a [`Symbol`] is defined as a global variable.
    pub fn is_defined(&self, symbol: Symbol) -> bool {
        self.values.contains_key(&symbol)
    }

    /// Assigns a [`Value`] to a [`Symbol`].
    pub fn assign(&mut self, symbol: Symbol, value: Value) {
        self.values.insert(symbol, value);
    }

    /// Returns a reference to a [`Value`] from its [`Symbol`].
    pub fn read(&self, symbol: Symbol) -> &Value {
        &self.values[&symbol]
    }
}
