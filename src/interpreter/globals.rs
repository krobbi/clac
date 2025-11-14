use std::collections::HashMap;

use crate::ir::Value;

use super::InterpretError;

/// A map of global variables.
pub struct Globals {
    /// The map of global variable names to [`Value`]s.
    values: HashMap<String, Value>,
}

impl Globals {
    /// Creates new `Globals`.
    pub fn new() -> Self {
        let mut globals = Self {
            values: HashMap::new(),
        };

        load_natives(&mut globals);
        globals
    }

    /// Returns `true` if a global variable is defined.
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
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

/// Loads the native functions into [`Globals`].
fn load_natives(globals: &mut Globals) {
    globals.set("sqrt", Value::Native(native_sqrt));
}

/// The native square root function.
fn native_sqrt(values: &[Value]) -> Result<Value, InterpretError> {
    match values {
        [Value::Number(value)] => Ok(Value::Number(value.sqrt())),
        [_] => Err(InterpretError::InvalidType),
        _ => Err(InterpretError::IncorrectCallArity),
    }
}
