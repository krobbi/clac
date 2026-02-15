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

    /// Returns an [`Iterator`] over the defined global variable [`Symbol`]s.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol> {
        self.values.keys().copied()
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
