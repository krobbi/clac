mod value;

use std::collections::HashMap;

use crate::ast::BinOp;

use super::{Cfg, Instruction, Label};

use self::value::Value;

// TODO: Preserve global variables between REPL lines. Consider grouping this
// global context with some other data - maybe implement a symbol table? Storing
// the output from Clac would also make it easier to implement some integration
// tests.
/// Interprets a [`Cfg`]. This module is isolated from the rest of Clac because
/// a lower-level bytecode representation is being considered. Register-based IR
/// is also being considered, but this is less likely to be implemented.
pub fn interpret_cfg(cfg: &Cfg) {
    let mut interpreter = Interpreter::new(cfg);
    interpreter.interpret_label(Label::default());
}

/// A structure that interprets a [`Cfg`].
struct Interpreter<'a> {
    /// The [`Cfg`].
    cfg: &'a Cfg,

    /// The stack of [`Value`]s.
    stack: Vec<Value>,

    /// The map of global variable names to [`Value`]s.
    globals: HashMap<String, Value>,
}

impl<'a> Interpreter<'a> {
    /// Creates a new `Interpreter`.
    fn new(cfg: &'a Cfg) -> Self {
        Self {
            cfg,
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    /// Interprets a [`Label`].
    fn interpret_label(&mut self, label: Label) {
        let block = self.cfg.block(label);

        for instruction in &block.instructions {
            self.interpret_instruction(instruction);
        }
    }

    /// Interprets an [`Instruction`].
    fn interpret_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::PushLiteral(literal) => self.push(literal.into()),
            Instruction::PushFunction(_label, _arity) => todo!("interpret instruction"),
            Instruction::Drop(count) => self.stack.truncate(self.stack.len() - count),
            Instruction::Print => println!("{}", self.pop()),
            Instruction::Binary(op) => {
                let rhs = self.pop_number();
                let lhs = self.pop_number();

                // TODO: Add error handling for division by zero.
                let result = match op {
                    BinOp::Add => lhs + rhs,
                    BinOp::Subtract => lhs - rhs,
                    BinOp::Multiply => lhs * rhs,
                    BinOp::Divide => lhs / rhs,
                };

                self.push(Value::Number(result));
            }
            Instruction::LoadLocal(offset) => self.push(self.stack[*offset].clone()),
            Instruction::StoreLocal(offset) => self.stack[*offset] = self.pop(),
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
        }
    }
}
