mod globals;
mod interpret_error;

pub use self::{globals::Globals, interpret_error::InterpretError};

use std::{cell::RefCell, rc::Rc};

use crate::ir::{BinOp, Body, Instruction, Ir, Value};

/// Interprets [`Ir`] with [`Globals`]. This function returns an
/// [`InterpretError`] if [`Ir`] could not be interpreted.
pub fn interpret_ir(ir: &Ir, globals: &mut Globals) -> Result<(), InterpretError> {
    let mut interpreter = Interpreter::new(&ir.0, RefCell::new(globals).into());
    interpreter.run()
}

/// A structure that interprets a [`Body`].
struct Interpreter<'a, 'b: 'a> {
    /// The [`Body`] to interpret.
    body: &'a Body,

    /// The [`Globals`].
    globals: Rc<RefCell<&'b mut Globals>>,

    /// The stack of [`Value`]s.
    stack: Vec<Value>,
}

impl<'a, 'b> Interpreter<'a, 'b> {
    /// Creates a new `Interpreter` from its [`Body`] and [`Globals`].
    fn new(body: &'a Body, globals: Rc<RefCell<&'b mut Globals>>) -> Self {
        let stack = Vec::new();
        Self {
            body,
            globals,
            stack,
        }
    }

    /// Runs the `Interpreter` until it is finished. This function returns an
    /// [`InterpretError`] if an error occurred.
    fn run(&mut self) -> Result<(), InterpretError> {
        for instruction in &self.body.0 {
            match instruction {
                Instruction::Push(value) => self.stack.push(value.clone()),
                Instruction::Drop => self.stack.truncate(self.stack.len() - 1),
                Instruction::Print => println!("{}", self.pop()),
                Instruction::LoadLocal(index) => self.stack.push(self.stack[*index].clone()),
                Instruction::LoadGlobal(name) => {
                    self.stack.push(self.globals.borrow().get(name).clone());
                }
                Instruction::StoreLocal(index) => self.stack[*index] = self.pop(),
                Instruction::StoreGlobal(name) => {
                    let value = self.pop();
                    self.globals.borrow_mut().set(name, value);
                }
                Instruction::Binary(op) => {
                    let rhs = self.pop_number()?;
                    let lhs = self.pop_number()?;

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

                    self.stack.push(Value::Number(result));
                }
                Instruction::Call(arity) => {
                    let arity = *arity;
                    let args = self.stack.split_off(self.stack.len() - arity);

                    let Value::Function(function) = self.pop() else {
                        return Err(InterpretError::InvalidType);
                    };

                    if function.0 != arity {
                        return Err(InterpretError::InvalidType);
                    }

                    let mut interpreter = Interpreter::new(&function.1, self.globals.clone());
                    interpreter.stack = args;
                    interpreter.run()?;
                    self.stack.push(interpreter.pop());
                }
            }
        }

        Ok(())
    }

    /// Pops a [`Value`] from the stack.
    fn pop(&mut self) -> Value {
        self.stack.pop().expect("stack should not be empty")
    }

    /// Pops a number [`Value`] from the stack and returns its underlying
    /// [`f64`]. This function returns an [`InterpretError`] if the [`Value`] is
    /// not a number.
    fn pop_number(&mut self) -> Result<f64, InterpretError> {
        if let Value::Number(value) = self.pop() {
            Ok(value)
        } else {
            Err(InterpretError::InvalidType)
        }
    }
}
