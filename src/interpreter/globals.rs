use std::collections::HashMap;

use super::{InterpretError, value::Value};

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

        globals.assign("sqrt", Value::Native(native_sqrt));
        globals
    }

    /// Returns `true` if a global variable is defined.
    pub fn is_defined(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Assigns a [`Value`] to a global variable.
    pub fn assign(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    /// Returns a reference to a global variable's [`Value`].
    pub fn read(&self, name: &str) -> &Value {
        &self.values[name]
    }
}

/// The native square root function.
fn native_sqrt(values: &[Value]) -> Result<Value, InterpretError> {
    match values {
        [Value::Number(value)] => Ok(Value::Number(value.sqrt())),
        [_] => Err(InterpretError::InvalidType),
        _ => Err(InterpretError::IncorrectCallArity),
    }
}
