mod interpret_error;
mod value;

use std::{collections::HashMap, mem, rc::Rc};

use crate::ast::BinOp;

use super::{Cfg, Exit, Instruction, Label};

use self::{
    interpret_error::InterpretError,
    value::{Closure, Function, Value},
};

// TODO: Preserve global variables between REPL lines. Consider grouping this
// global context with some other data - maybe implement a symbol table? Storing
// the output from Clac would also make it easier to implement some integration
// tests.
/// Interprets a [`Cfg`]. This module is isolated from the rest of Clac because
/// a lower-level bytecode representation is being considered. Register-based IR
/// is also being considered, but this is less likely to be implemented.
pub fn interpret_cfg(cfg: &Cfg) {
    let mut interpreter = Interpreter::new(cfg);

    if let Err(error) = interpreter.interpret() {
        eprintln!("Error: {error}");
    }
}

/// A structure that interprets a [`Cfg`].
struct Interpreter<'a> {
    /// The [`Cfg`].
    cfg: &'a Cfg,

    /// The stack of [`Value`]s.
    stack: Vec<Value>,

    /// The stack offset to the current stack frame.
    frame: usize,

    /// The map of global variable names to [`Value`]s.
    globals: HashMap<String, Value>,

    /// The stack of upvalues.
    upvalues: Vec<Rc<Value>>,

    /// The stack of [`Return`]s.
    returns: Vec<Return>,
}

impl<'a> Interpreter<'a> {
    /// Creates a new `Interpreter`.
    fn new(cfg: &'a Cfg) -> Self {
        Self {
            cfg,
            stack: Vec::new(),
            frame: 0,
            globals: HashMap::new(),
            upvalues: Vec::new(),
            returns: Vec::new(),
        }
    }

    /// Interprets the [`Cfg`] until execution halts. This function returns an
    /// [`InterpretError`] if an error occurred.
    fn interpret(&mut self) -> Result<(), InterpretError> {
        let mut label = Label::default();

        while let Some(next_label) = self.interpret_label(label)? {
            label = next_label;
        }

        Ok(())
    }

    /// Interprets a [`Label`] and returns the next [`Label`] to branch to. This
    /// function returns [`None`] if execution should halt. This function also
    /// returns an [`InterpretError`] if an error occurred.
    fn interpret_label(&mut self, label: Label) -> Result<Option<Label>, InterpretError> {
        let block = self.cfg.block(label);

        for instruction in &block.instructions {
            self.interpret_instruction(instruction)?;
        }

        self.interpret_exit(&block.exit)
    }

    /// Interprets an [`Instruction`]. This function returns an
    /// [`InterpretError`] if an error occurred.
    fn interpret_instruction(&mut self, instruction: &Instruction) -> Result<(), InterpretError> {
        match instruction {
            Instruction::PushLiteral(literal) => self.push(literal.into()),
            Instruction::PushFunction(label, arity) => self.push(Value::Function(
                Function {
                    label: *label,
                    arity: *arity,
                }
                .into(),
            )),
            Instruction::Drop(count) => self.stack.truncate(self.stack.len() - count),
            Instruction::Print => println!("{}", self.pop()),
            Instruction::Binary(op) => {
                let rhs = self.pop_number()?;
                let lhs = self.pop_number()?;

                let result = match op {
                    BinOp::Add => lhs + rhs,
                    BinOp::Subtract => lhs - rhs,
                    BinOp::Multiply => lhs * rhs,
                    BinOp::Divide => {
                        if !rhs.is_normal() {
                            return Err(InterpretError::DivideByZero);
                        }

                        lhs / rhs
                    }
                };

                self.push(Value::Number(result));
            }
            Instruction::LoadLocal(offset) => self.push(self.stack[self.frame + *offset].clone()),
            Instruction::StoreLocal(offset) => self.stack[self.frame + *offset] = self.pop(),
            Instruction::LoadGlobal(name) => self.push(self.globals[name].clone()),
            Instruction::StoreGlobal(name) => {
                let value = self.pop();
                self.globals.insert(name.to_owned(), value);
            }
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
                    function: *function,
                    upvalues: self.upvalues.clone(),
                };

                self.push(Value::Closure(closure.into()));
            }
        }

        Ok(())
    }

    /// Interprets an [`Exit`] and returns the next [`Label`] to branch to. This
    /// function returns [`None`] if execution should halt. This function also
    /// returns an [`InterpretError`] if an error occurred.
    fn interpret_exit(&mut self, exit: &Exit) -> Result<Option<Label>, InterpretError> {
        let label = match exit {
            Exit::Halt => return Ok(None),
            Exit::Call(arity, return_label) => {
                let mut return_data = Return {
                    label: *return_label,
                    frame: self.frame,
                    upvalues: None,
                };

                let arity = *arity;
                self.frame = self.stack.len() - arity;

                let function = match &self.stack[self.frame - 1] {
                    Value::Number(_) => return Err(InterpretError::CalledNonFunction),
                    Value::Function(function) => function,
                    Value::Closure(closure) => {
                        let outer_upvalues =
                            mem::replace(&mut self.upvalues, closure.upvalues.clone());

                        return_data.upvalues = Some(outer_upvalues);
                        &closure.function
                    }
                };

                if arity != function.arity {
                    return Err(InterpretError::IncorrectCallArity);
                }

                self.returns.push(return_data);
                function.label
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

                return_data.label
            }
        };

        Ok(Some(label))
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
}

/// Data for returning from a function.
struct Return {
    /// The [`Label`] to return to.
    label: Label,

    /// The stack offset of the return stack frame.
    frame: usize,

    /// The optional stack of upvalues to return to.
    upvalues: Option<Vec<Rc<Value>>>,
}
