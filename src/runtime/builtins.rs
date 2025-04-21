use super::{EvalResult, runtime_error::RuntimeError, value::Value};

/// The built-in square root function.
pub fn builtin_sqrt(args: &[Value]) -> EvalResult {
    match args[..] {
        [Value::Number(value)] => Ok(Some(Value::Number(value.sqrt()))),
        [_] => Err(RuntimeError::IncorrectArgTypes),
        _ => Err(RuntimeError::IncorrectArgCount),
    }
}
