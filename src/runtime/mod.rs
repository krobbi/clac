mod runtime_error;
mod value_ops;

use std::collections::HashMap;

use runtime_error::RuntimeError;

use crate::ast::{BinOp, Expr, Value};

/// The runtime environment of a Clac program.
pub struct Runtime {
    /// The global variables.
    variables: HashMap<String, Value>,
}

impl Runtime {
    /// Creates a new runtime environment.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Executes a top-level expression.
    pub fn execute_expr(&mut self, expr: Expr) -> Result<(), RuntimeError> {
        match self.eval_expr(expr)? {
            Value::Void => {}
            value @ Value::Number(_) => println!("{value}"),
        }

        Ok(())
    }

    /// Evaluates an expression.
    fn eval_expr(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(value) => Ok(value),
            Expr::Ident(name) => self.eval_ident(name),
            Expr::Block(exprs) => self.eval_block(exprs),
            Expr::Negate(rhs) => self.eval_negate(*rhs),
            Expr::Binary { lhs, op, rhs } => self.eval_binary(*lhs, op, *rhs),
        }
    }

    /// Evaluates an expression as an argument that cannot be void.
    fn eval_arg(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match self.eval_expr(expr) {
            Ok(Value::Void) => Err(RuntimeError::VoidArgument),
            value => value,
        }
    }

    /// Evaluates an identifier expression.
    fn eval_ident(&self, name: String) -> Result<Value, RuntimeError> {
        match self.variables.get(&name).cloned() {
            None => Err(RuntimeError::UndefinedVariable(name)),
            Some(value) => Ok(value),
        }
    }

    /// Evaluates a block expression.
    fn eval_block(&mut self, mut exprs: Vec<Expr>) -> Result<Value, RuntimeError> {
        match exprs.pop() {
            None => Ok(Value::Void),
            Some(last_expr) => {
                for expr in exprs {
                    self.eval_expr(expr)?;
                }

                self.eval_expr(last_expr)
            }
        }
    }

    /// Evaluates a unary negation expression.
    fn eval_negate(&mut self, rhs: Expr) -> Result<Value, RuntimeError> {
        let rhs = self.eval_expr(rhs)?;
        -rhs
    }

    /// Evaluates a binary expression.
    fn eval_binary(&mut self, lhs: Expr, op: BinOp, rhs: Expr) -> Result<Value, RuntimeError> {
        // Assignment is handled as a special case because its operands cannot
        // be eagerly evaluated. The left-hand-side is an assignment target, not
        // an actual expression. The right-hand-side is only evaluated after the
        // target is validated as a variable.
        if let BinOp::Assign = op {
            return self.eval_assignment(lhs, rhs);
        }

        let lhs = self.eval_expr(lhs)?;
        let rhs = self.eval_expr(rhs)?;

        match op {
            BinOp::Assign => unreachable!(),
            BinOp::Add => lhs + rhs,
            BinOp::Sub => lhs - rhs,
            BinOp::Mul => lhs * rhs,
            BinOp::Div => lhs / rhs,
        }
    }

    /// Evaluates an assignment expression.
    fn eval_assignment(&mut self, target: Expr, source: Expr) -> Result<Value, RuntimeError> {
        if let Expr::Ident(name) = target {
            let value = self.eval_arg(source)?;
            self.variables.insert(name, value);
            Ok(Value::Void)
        } else {
            Err(RuntimeError::NonVariableAssignment)
        }
    }
}
