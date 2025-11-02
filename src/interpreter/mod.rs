mod interpret_error;

pub use self::interpret_error::InterpretError;

use crate::ir::{BinOp, Instruction, Ir, UnOp, Value};

/// Interprets [`Ir`]. This function returns an [`InterpretError`] if [`Ir`]
/// could not be interpreted.
pub fn interpret_ir(ir: &Ir) -> Result<(), InterpretError> {
    let mut stack = Stack::new();

    for instruction in &ir.0.0 {
        match instruction {
            Instruction::Push(value) => stack.push(value.clone()),
            Instruction::PushGlobal(..) => todo!("interpreting `Instruction::PushGlobal`"),
            Instruction::PushLocal(index) => stack.push(stack.get_local(*index).clone()),
            Instruction::StoreGlobal(..) => todo!("interpreting `Instruction::StoreGlobal`"),
            Instruction::StoreLocal(index) => {
                let value = stack.pop();
                stack.set_local(*index, value)?;
            }
            Instruction::Pop => {
                stack.pop();
            }
            Instruction::Print => print_value(&stack.pop()),
            Instruction::Unary(op) => {
                let value = stack.pop_number()?;

                let result = match op {
                    UnOp::Negate => -value,
                };

                stack.push(Value::Number(result));
            }
            Instruction::Binary(op) => {
                let rhs = stack.pop_number()?;
                let lhs = stack.pop_number()?;

                let result = match op {
                    BinOp::Add => lhs + rhs,
                    BinOp::Subtract => lhs - rhs,
                    BinOp::Multiply => lhs * rhs,
                    BinOp::Divide => {
                        if rhs.is_subnormal() || rhs == 0.0 {
                            return Err(InterpretError::DivideByZero);
                        }

                        lhs / rhs
                    }
                };

                stack.push(Value::Number(result));
            }
            Instruction::Halt => break,
        }
    }

    Ok(())
}

/// A stack of [`Value`]s.
struct Stack {
    /// The [`Value`]s.
    values: Vec<Value>,
}

impl Stack {
    /// Creates a new `Stack`.
    fn new() -> Self {
        let values = Vec::new();
        Self { values }
    }

    /// Pushes a [`Value`] to the `Stack`.
    fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    /// Pops a [`Value`] from the `Stack`.
    fn pop(&mut self) -> Value {
        self.values.pop().expect("stack should not be empty")
    }

    /// Pops a number [`Value`] from the `Stack` and returns its underlying
    /// [`f64`]. This function returns an [`InterpretError`] if the [`Value`] is
    /// not a number.
    fn pop_number(&mut self) -> Result<f64, InterpretError> {
        if let Value::Number(value) = self.pop() {
            Ok(value)
        } else {
            Err(InterpretError::InvalidType)
        }
    }

    /// Returns a reference to a local variable's [`Value`].
    fn get_local(&self, index: usize) -> &Value {
        &self.values[index]
    }

    /// Sets a local variable's [`Value`]. This function returns an
    /// [`InterpretError`] if the [`Value`] is void.
    fn set_local(&mut self, index: usize, value: Value) -> Result<(), InterpretError> {
        if let Value::Void = value {
            Err(InterpretError::InvalidType)
        } else {
            self.values[index] = value;
            Ok(())
        }
    }
}

/// Prints a [`Value`] if it is not void.
fn print_value(value: &Value) {
    if let Value::Void = value {
        return;
    }

    println!("{value}");
}
