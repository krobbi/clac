mod errors;
mod globals;
mod native;
mod value;

use thiserror::Error;

pub use self::{globals::Globals, native::install_natives};

use std::{mem, rc::Rc};

use crate::cfg::{BasicBlock, Cfg, Function, Instruction, Label, Terminator};

use self::{
    errors::ErrorKind,
    value::{Closure, Value},
};

#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct InterpretError(ErrorKind);

/// Interprets a [`Cfg`] with [`Globals`]. This function returns an
/// [`InterpretError`] if an error occurred.
pub fn interpret_cfg(cfg: &Cfg, globals: &mut Globals) -> Result<(), InterpretError> {
    let mut interpreter = Interpreter::new(globals);
    let mut called_functions: Vec<Rc<Function>> = Vec::new();
    let mut label = Label::default();

    loop {
        let basic_block = called_functions
            .last()
            .map_or(cfg, |f| &f.cfg)
            .basic_block(label);

        let flow = interpreter.interpret_basic_block(basic_block)?;

        match flow {
            Flow::Halt => break,
            Flow::Jump(target_label) => label = target_label,
            Flow::Call(function) => {
                called_functions.push(function);
                label = Label::default();
            }
            Flow::Return(return_label) => {
                called_functions.truncate(called_functions.len() - 1);
                label = return_label;
            }
        }
    }

    Ok(())
}

/// A structure which interprets a [`Cfg`].
struct Interpreter<'glb> {
    /// The stack of [`Value`]s.
    stack: Vec<Value>,

    /// The stack offset to the current stack frame.
    frame: usize,

    /// The [`Globals`].
    globals: &'glb mut Globals,

    /// The stack of upvars.
    upvars: Vec<Rc<Value>>,

    /// The stack of [`Return`]s.
    returns: Vec<Return>,
}

impl<'glb> Interpreter<'glb> {
    /// Creates a new `Interpreter` from [`Globals`].
    const fn new(globals: &'glb mut Globals) -> Self {
        Self {
            stack: Vec::new(),
            frame: 0,
            globals,
            upvars: Vec::new(),
            returns: Vec::new(),
        }
    }

    /// Interprets a [`BasicBlock`] and returns a [`Flow`]. This function
    /// returns an [`InterpretError`] if an error occurred.
    fn interpret_basic_block(&mut self, basic_block: &BasicBlock) -> Result<Flow, InterpretError> {
        for instruction in &basic_block.instructions {
            self.interpret_instruction(instruction)?;
        }

        self.interpret_terminator(&basic_block.terminator)
    }

    /// Interprets an [`Instruction`]. This function returns an
    /// [`InterpretError`] if an error occurred.
    #[expect(
        clippy::too_many_lines,
        reason = "function contains a single match expression"
    )]
    fn interpret_instruction(&mut self, instruction: &Instruction) -> Result<(), InterpretError> {
        match instruction {
            Instruction::PushLiteral(literal) => self.push((*literal).into()),
            Instruction::PushFunction(function) => self.push(Value::Function(Rc::clone(function))),
            Instruction::PushGlobal(symbol) => self.push(self.globals.read(*symbol).clone()),
            Instruction::PushLocal(offset) => self.push(self.stack[self.frame + *offset].clone()),
            Instruction::PushUpvar(offset) => self.push((*self.upvars[*offset]).clone()),
            Instruction::Pop(count) => self.stack.truncate(self.stack.len() - count),
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
                    return Err(ErrorKind::DivideByZero.into());
                }

                self.push(Value::Number(lhs / rhs));
            }
            Instruction::Power => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Number(lhs.powf(rhs)));
            }
            Instruction::Equal => {
                let rhs = self.pop();
                let lhs = self.pop();

                if !lhs.matches_value_type(&rhs) {
                    return Err(ErrorKind::InvalidType.into());
                }

                self.push(Value::Bool(lhs == rhs));
            }
            Instruction::NotEqual => {
                let rhs = self.pop();
                let lhs = self.pop();

                if !lhs.matches_value_type(&rhs) {
                    return Err(ErrorKind::InvalidType.into());
                }

                self.push(Value::Bool(lhs != rhs));
            }
            Instruction::Less => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Bool(lhs < rhs));
            }
            Instruction::LessEqual => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Bool(lhs <= rhs));
            }
            Instruction::Greater => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Bool(lhs > rhs));
            }
            Instruction::GreaterEqual => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;
                self.push(Value::Bool(lhs >= rhs));
            }
            Instruction::StoreGlobal(symbol) => {
                let value = self.pop();
                self.globals.assign(*symbol, value);
            }
            Instruction::StoreLocal(offset) => self.stack[self.frame + *offset] = self.pop(),
            Instruction::DefineUpvar => {
                let value = self.pop();
                self.upvars.push(value.into());
            }
            Instruction::PopUpvars(count) => self.upvars.truncate(self.upvars.len() - count),
            Instruction::IntoClosure => {
                let Value::Function(function) = self.pop() else {
                    unreachable!("value should be a function");
                };

                let closure = Closure {
                    function,
                    upvars: self.upvars.clone(),
                };

                self.push(Value::Closure(closure.into()));
            }
        }

        Ok(())
    }

    /// Interprets an [`Terminator`] and returns a [`Flow`]. This function
    /// returns an [`InterpretError`] if an error occurred.
    fn interpret_terminator(&mut self, terminator: &Terminator) -> Result<Flow, InterpretError> {
        let branch = match terminator {
            Terminator::Halt => Flow::Halt,
            Terminator::Jump(label) => Flow::Jump(*label),
            Terminator::Branch(then_label, else_label) => {
                let label = if self.pop_bool()? {
                    *then_label
                } else {
                    *else_label
                };

                Flow::Jump(label)
            }
            Terminator::Call(arity, return_label) => {
                let mut return_data = Return {
                    label: *return_label,
                    frame: self.frame,
                    upvars: None,
                };

                let arity = *arity;
                self.frame = self.stack.len() - arity - 1;

                let function = match &self.stack[self.frame] {
                    Value::Function(function) => Rc::clone(function),
                    Value::Closure(closure) => {
                        let outer_upvars = mem::replace(&mut self.upvars, closure.upvars.clone());
                        return_data.upvars = Some(outer_upvars);
                        Rc::clone(&closure.function)
                    }
                    Value::Native(native) => {
                        let return_value = native.call(&self.stack[self.frame + 1..])?;
                        self.stack.truncate(self.frame);
                        self.push(return_value);
                        self.frame = return_data.frame;
                        return Ok(Flow::Jump(*return_label));
                    }
                    _ => return Err(ErrorKind::CalledNonFunction.into()),
                };

                if arity != function.arity {
                    return Err(ErrorKind::IncorrectCallArity.into());
                }

                self.returns.push(return_data);
                Flow::Call(function)
            }
            Terminator::Return => {
                let return_value = self.pop();
                self.stack.truncate(self.frame);
                self.push(return_value);
                let return_data = self
                    .returns
                    .pop()
                    .expect("return stack should not be empty");

                self.frame = return_data.frame;

                if let Some(upvars) = return_data.upvars {
                    self.upvars = upvars;
                }

                Flow::Return(return_data.label)
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
            _ => Err(ErrorKind::InvalidType.into()),
        }
    }

    /// Pops a boolean [`Value`] from the stack and returns its underlying
    /// [`bool`]. This function returns an [`InterpretError`] if the [`Value`]
    /// is not a Boolean value.
    fn pop_bool(&mut self) -> Result<bool, InterpretError> {
        match self.pop() {
            Value::Bool(value) => Ok(value),
            _ => Err(ErrorKind::InvalidType.into()),
        }
    }
}

/// Control flow after interpreting a [`Terminator`].
enum Flow {
    /// Halts execution.
    Halt,

    /// Jumps to a [`Label`].
    Jump(Label),

    /// Calls a [`Function`].
    Call(Rc<Function>),

    /// Returns to a [`Label`] from a [`Function`].
    Return(Label),
}

/// Data for returning from a function.
struct Return {
    /// The [`Label`] to return to.
    label: Label,

    /// The stack offset of the return stack frame.
    frame: usize,

    /// The optional stack of upvars to restore.
    upvars: Option<Vec<Rc<Value>>>,
}
