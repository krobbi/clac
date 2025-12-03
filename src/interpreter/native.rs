use super::{Globals, InterpretError, value::Value};

/// A native function.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Native {
    /// Returns the square root of `n`.
    ///
    /// Signature: `sqrt(n: number) -> number`
    Sqrt,
}

impl Native {
    /// Calls the `Native` and returns its return [`Value`]. This function
    /// returns an [`InterpretError`] if an error occurred.
    pub fn call(self, args: &[Value]) -> Result<Value, InterpretError> {
        self.fn_ptr()(args)
    }

    /// Returns the `Native`'s name.
    fn name(self) -> &'static str {
        match self {
            Self::Sqrt => "sqrt",
        }
    }

    /// Returns the `Native`'s function pointer.
    fn fn_ptr(self) -> fn(&[Value]) -> Result<Value, InterpretError> {
        match self {
            Self::Sqrt => native_sqrt,
        }
    }
}

/// Installs [`Native`] variables into [`Globals`].
pub fn install_natives(globals: &mut Globals) {
    install_native(Native::Sqrt, globals);
}

/// Installs a [`Native`] variable into [`Globals`].
fn install_native(native: Native, globals: &mut Globals) {
    globals.assign(native.name(), Value::Native(native));
}

/// The native `sqrt` function.
fn native_sqrt(args: &[Value]) -> Result<Value, InterpretError> {
    match args {
        [Value::Number(value)] => Ok(Value::Number(value.sqrt())),
        [_] => Err(InterpretError::InvalidType),
        _ => Err(InterpretError::IncorrectCallArity),
    }
}
