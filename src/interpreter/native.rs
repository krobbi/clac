use super::{Globals, InterpretError, value::Value};

/// A native function.
#[expect(
    clippy::doc_paragraphs_missing_punctuation,
    reason = "function signature documentation"
)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Native {
    /// Prints `f`'s control flow graph as pseudo-assembly and returns `f`.
    ///
    /// Signature: `__dump(f: function) -> function`
    Dump,

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
    const fn name(self) -> &'static str {
        match self {
            Self::Dump => "__dump",
            Self::Sqrt => "sqrt",
        }
    }

    /// Returns the `Native`'s function pointer.
    fn fn_ptr(self) -> fn(&[Value]) -> Result<Value, InterpretError> {
        match self {
            Self::Dump => native_dump,
            Self::Sqrt => native_sqrt,
        }
    }
}

/// Installs [`Native`] variables into [`Globals`].
pub fn install_natives(globals: &mut Globals) {
    install_native(Native::Dump, globals);
    install_native(Native::Sqrt, globals);
}

/// Installs a [`Native`] variable into [`Globals`].
fn install_native(native: Native, globals: &mut Globals) {
    globals.assign(native.name(), Value::Native(native));
}

/// The native `__dump` function.
fn native_dump(args: &[Value]) -> Result<Value, InterpretError> {
    match args {
        [Value::Function(function)] => {
            println!(
                "[function with {} parameter(s)]\n{}",
                function.arity, function.cfg,
            );

            Ok(args[0].clone())
        }
        [Value::Closure(closure)] => {
            println!(
                "[closure with {} parameter(s) and {} upvalue(s)]",
                closure.function.arity,
                closure.upvalues.len()
            );

            for (offset, upvalue) in closure.upvalues.iter().enumerate() {
                println!("{:8}[{offset}] = {upvalue}", "");
            }

            println!("{}", closure.function.cfg);
            Ok(args[0].clone())
        }
        [Value::Native(native)] => {
            println!("[native '{}' function]", native.name());
            Ok(args[0].clone())
        }
        [_] => Err(InterpretError::InvalidType),
        _ => Err(InterpretError::IncorrectCallArity),
    }
}

/// The native `sqrt` function.
fn native_sqrt(args: &[Value]) -> Result<Value, InterpretError> {
    match args {
        [Value::Number(value)] => Ok(Value::Number(value.sqrt())),
        [_] => Err(InterpretError::InvalidType),
        _ => Err(InterpretError::IncorrectCallArity),
    }
}
