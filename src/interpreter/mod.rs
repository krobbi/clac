mod globals;
mod interpret_error;
mod native;
mod value;

pub use self::{globals::Globals, interpret_error::InterpretError, native::install_natives};

use std::{mem, rc::Rc};

use crate::cfg::{Cfg, Exit, Function, Instruction, Label};

use self::value::{Closure, Value};

/// Interprets a [`Cfg`] with [`Globals`]. This function returns an
/// [`InterpretError`] if an error occurred.
pub fn interpret_cfg(cfg: &Cfg, globals: &mut Globals) -> Result<(), InterpretError> {
    let mut interpreter = Interpreter::new();
    let mut called_functions: Vec<Rc<Function>> = Vec::new();
    let mut label = Label::default();

    loop {
        let block = match called_functions.last() {
            None => cfg.block(label),
            Some(function) => function.cfg.block(label),
        };

        for instruction in &block.instructions {
            interpreter.interpret_instruction(instruction, globals)?;
        }

        match interpreter.interpret_exit(&block.exit)? {
            Branch::Halt => break,
            Branch::Jump(target_label) => label = target_label,
            Branch::Call(function) => {
                called_functions.push(function);
                label = Label::default();
            }
            Branch::Return(return_label) => {
                called_functions.truncate(called_functions.len() - 1);
                label = return_label;
            }
        }
    }

    Ok(())
}

/// A structure that interprets a [`Cfg`].
#[derive(Default)]
struct Interpreter {
    /// The stack of [`Value`]s.
    stack: Vec<Value>,

    /// The stack offset to the current stack frame.
    frame: usize,

    /// The stack of upvalues.
    upvalues: Vec<Rc<Value>>,

    /// The stack of [`Return`]s.
    returns: Vec<Return>,
}

impl Interpreter {
    /// Creates a new `Interpreter`.
    fn new() -> Self {
        Self::default()
    }

    /// Interprets an [`Instruction`] with [`Globals`]. This function returns an
    /// [`InterpretError`] if an error occurred.
    fn interpret_instruction(
        &mut self,
        instruction: &Instruction,
        globals: &mut Globals,
    ) -> Result<(), InterpretError> {
        match instruction {
            Instruction::PushLiteral(literal) => self.push(literal.into()),
            Instruction::PushFunction(function) => self.push(Value::Function(function.clone())),
            Instruction::Drop(count) => self.stack.truncate(self.stack.len() - count),
            Instruction::Print => println!("{}", self.pop()),
            Instruction::Negate => {
                let rhs = self.pop_number()?;
                self.push(Value::Number(-rhs));
            }
            Instruction::Not => {
                let rhs = self.pop_bool()?;
                self.push(Value::Bool(!rhs));
            }
            Instruction::Add => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Number(lhs + rhs));
            }
            Instruction::Subtract => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Number(lhs - rhs));
            }
            Instruction::Multiply => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Number(lhs * rhs));
            }
            Instruction::Divide => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;

                if !rhs.is_normal() {
                    return Err(InterpretError::DivideByZero);
                }

                self.push(Value::Number(lhs / rhs));
            }
            Instruction::LoadLocal(offset) => self.push(self.stack[self.frame + *offset].clone()),
            Instruction::StoreLocal(offset) => self.stack[self.frame + *offset] = self.pop(),
            Instruction::LoadGlobal(name) => self.push(globals.read(name).clone()),
            Instruction::StoreGlobal(name) => globals.assign(name, self.pop()),
            Instruction::DefineUpvalue => {
                let value = self.pop();
                self.upvalues.push(value.into());
            }
            Instruction::LoadUpvalue(offset) => self.stack.push((*self.upvalues[*offset]).clone()),
            Instruction::DropUpvalues(count) => self.upvalues.truncate(self.upvalues.len() - count),
            Instruction::IntoClosure => {
                let Value::Function(function) = self.pop() else {
                    panic!("value should be a function");
                };

                let closure = Closure {
                    function,
                    upvalues: self.upvalues.clone(),
                };

                self.push(Value::Closure(closure.into()));
            }
        }

        Ok(())
    }

    /// Interprets an [`Exit`] and returns the next [`Branch`]. This function
    /// returns an [`InterpretError`] if an error occurred.
    fn interpret_exit(&mut self, exit: &Exit) -> Result<Branch, InterpretError> {
        let branch = match exit {
            Exit::Halt => Branch::Halt,
            Exit::Call(arity, return_label) => {
                let mut return_data = Return {
                    label: *return_label,
                    frame: self.frame,
                    upvalues: None,
                };

                let arity = *arity;
                self.frame = self.stack.len() - arity;

                let function = match &self.stack[self.frame - 1] {
                    Value::Function(function) => function.clone(),
                    Value::Closure(closure) => {
                        let outer_upvalues =
                            mem::replace(&mut self.upvalues, closure.upvalues.clone());

                        return_data.upvalues = Some(outer_upvalues);
                        closure.function.clone()
                    }
                    Value::Native(native) => {
                        let return_value = native.call(&self.stack[self.frame..])?;
                        self.stack.truncate(self.frame);
                        self.frame = return_data.frame;
                        *self.stack.last_mut().expect("stack should not be empty") = return_value;
                        return Ok(Branch::Jump(*return_label));
                    }
                    _ => return Err(InterpretError::CalledNonFunction),
                };

                if arity != function.arity {
                    return Err(InterpretError::IncorrectCallArity);
                }

                self.returns.push(return_data);
                Branch::Call(function)
            }
            Exit::Return => {
                let return_value = self.pop();
                self.stack.truncate(self.frame);
                *self.stack.last_mut().expect("stack should not be empty") = return_value;
                let return_data = self
                    .returns
                    .pop()
                    .expect("return stack should not be empty");

                self.frame = return_data.frame;

                if let Some(upvalues) = return_data.upvalues {
                    self.upvalues = upvalues;
                }

                Branch::Return(return_data.label)
            }
        };

        Ok(branch)
    }

    /// Pushes a [`Value`] to the stack.
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Pops a [`Value`] from the stack.
    fn pop(&mut self) -> Value {
        self.stack.pop().expect("stack should not be empty")
    }

    /// Pops a number [`Value`] from the stack and returns its underlying
    /// [`f64`]. This function returns an [`InterpretError`] if the [`Value`] is
    /// not a number.
    fn pop_number(&mut self) -> Result<f64, InterpretError> {
        match self.pop() {
            Value::Number(value) => Ok(value),
            _ => Err(InterpretError::InvalidType),
        }
    }

    /// Pops a boolean [`Value`] from the stack and returns its underlying
    /// [`bool`]. This function returns an [`InterpretError`] if the [`Value`]
    /// is not a Boolean value.
    fn pop_bool(&mut self) -> Result<bool, InterpretError> {
        match self.pop() {
            Value::Bool(value) => Ok(value),
            _ => Err(InterpretError::InvalidType),
        }
    }
}

/// Data for returning from a function.
struct Return {
    /// The [`Label`] to return to.
    label: Label,

    /// The stack offset of the return stack frame.
    frame: usize,

    /// The optional stack of upvalues to restore.
    upvalues: Option<Vec<Rc<Value>>>,
}

/// A branch to take after interpreting an [`Exit`].
enum Branch {
    /// Halts execution.
    Halt,

    /// Jumps to a [`Label`].
    Jump(Label),

    /// Calls a [`Function`].
    Call(Rc<Function>),

    /// Returns to a [`Label`] from a [`Function`].
    Return(Label),
}
