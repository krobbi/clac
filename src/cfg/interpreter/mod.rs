mod value;

use std::collections::HashMap;

use crate::ast::BinOp;

use super::{Cfg, Exit, Instruction, Label};

use self::value::{Function, Value};

// TODO: Preserve global variables between REPL lines. Consider grouping this
// global context with some other data - maybe implement a symbol table? Storing
// the output from Clac would also make it easier to implement some integration
// tests.
/// Interprets a [`Cfg`]. This module is isolated from the rest of Clac because
/// a lower-level bytecode representation is being considered. Register-based IR
/// is also being considered, but this is less likely to be implemented.
pub fn interpret_cfg(cfg: &Cfg) {
    let mut interpreter = Interpreter::new(cfg);
    interpreter.interpret();
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
            returns: Vec::new(),
        }
    }

    /// Interprets the [`Cfg`] until execution halts.
    fn interpret(&mut self) {
        let mut label = Label::default();

        while let Some(next_label) = self.interpret_label(label) {
            label = next_label;
        }
    }

    /// Interprets a [`Label`] and returns the next [`Label`] to branch to. This
    /// function returns [`None`] if execution should halt.
    fn interpret_label(&mut self, label: Label) -> Option<Label> {
        let block = self.cfg.block(label);

        for instruction in &block.instructions {
            self.interpret_instruction(instruction);
        }

        self.interpret_exit(&block.exit)
    }

    /// Interprets an [`Instruction`].
    fn interpret_instruction(&mut self, instruction: &Instruction) {
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
                let rhs = self.pop_number();
                let lhs = self.pop_number();

                let result = match op {
                    BinOp::Add => lhs + rhs,
                    BinOp::Subtract => lhs - rhs,
                    BinOp::Multiply => lhs * rhs,
                    BinOp::Divide => {
                        if !rhs.is_normal() {
                            todo!("error handling for division by zero");
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
            Instruction::DefineUpvalue => todo!("interpret instruction"),
            Instruction::LoadUpvalue(_offset) => todo!("interpret instruction"),
            Instruction::DropUpvalues(_count) => todo!("interpret instruction"),
            Instruction::IntoClosure => todo!("interpret instruction"),
        }
    }

    /// Interprets an [`Exit`] and returns the next [`Label`] to branch to. This
    /// function returns [`None`] if execution should halt.
    fn interpret_exit(&mut self, exit: &Exit) -> Option<Label> {
        match exit {
            Exit::Halt => None,
            Exit::Call(arity, return_label) => {
                self.returns.push(Return {
                    label: *return_label,
                    frame: self.frame,
                });

                let arity = *arity;
                self.frame = self.stack.len() - arity;

                let Value::Function(function) = &self.stack[self.frame - 1] else {
                    todo!("error handling for non-function calls");
                };

                if arity != function.arity {
                    todo!("error handling for incorrect call arity");
                }

                Some(function.label)
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
                Some(return_data.label)
            }
        }
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
    /// [`f64`].
    fn pop_number(&mut self) -> f64 {
        match self.pop() {
            Value::Number(value) => value,
            Value::Function(_) => todo!("add error handling for non-number values"),
        }
    }
}

/// Data for returning from a function.
struct Return {
    /// The [`Label`] to return to.
    label: Label,

    /// The stack offset of the return stack frame.
    frame: usize,
}
