mod globals;
mod interpret_error;
mod value;

pub use self::{globals::Globals, interpret_error::InterpretError};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    decl_table::DeclId,
    ir::{BinOp, Body, Closure, Function, Instruction, Ir, Value},
};

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

    /// The upvalues.
    upvalues: HashMap<DeclId, Rc<Value>>,
}

impl<'a, 'b> Interpreter<'a, 'b> {
    /// Creates a new `Interpreter` from its [`Body`] and [`Globals`].
    fn new(body: &'a Body, globals: Rc<RefCell<&'b mut Globals>>) -> Self {
        Self {
            body,
            globals,
            stack: Vec::new(),
            upvalues: HashMap::new(),
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
                Instruction::IntoClosure => {
                    let Value::Function(function) = self.pop() else {
                        return Err(InterpretError::InvalidType);
                    };

                    let closure = Closure {
                        function,
                        upvalues: self.upvalues.clone(),
                    };

                    self.stack.push(Value::Closure(closure.into()));
                }
                Instruction::Call(arity) => {
                    let arity = *arity;
                    let args = self.stack.split_off(self.stack.len() - arity);

                    let return_value = match self.pop() {
                        Value::Number(_) => Err(InterpretError::CalledNonFunction),
                        Value::Function(function) => {
                            self.call_function(&function, args, self.upvalues.clone())
                        }
                        Value::Closure(closure) => {
                            self.call_function(&closure.function, args, closure.upvalues.clone())
                        }
                        Value::Native(function) => function(&args),
                    }?;

                    self.stack.push(return_value);
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

    /// Calls a [`Function`] with arguments and upvalues and returns its return
    /// [`Value`]. This function returns an [`InterpretError`] if the incorrect
    /// number of arguments were passed or if an error occurred while
    /// interpreting the [`Function`].
    fn call_function(
        &self,
        function: &'a Function,
        args: Vec<Value>,
        upvalues: HashMap<DeclId, Rc<Value>>,
    ) -> Result<Value, InterpretError> {
        if function.arity != args.len() {
            return Err(InterpretError::IncorrectCallArity);
        }

        let mut interpreter = Self::new(&function.body, self.globals.clone());
        interpreter.stack = args;
        interpreter.upvalues = upvalues;
        interpreter.run()?;
        Ok(interpreter.pop())
    }
}
