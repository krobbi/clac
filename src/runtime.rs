use std::{collections::HashMap, error, fmt};

use crate::{bin_op::BinOp, expr::Expr};

/// The runtime environment of a Clac program.
pub struct Runtime {
    /// The global variables.
    variables: HashMap<String, f64>,
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
        let value = self.eval_expr(expr)?;
        println!("{value}");
        Ok(())
    }

    /// Evaluates an expression.
    fn eval_expr(&mut self, expr: Expr) -> Result<f64, RuntimeError> {
        match expr {
            Expr::Literal(value) => Ok(value),
            Expr::Ident(name) => self.eval_ident(name),
            Expr::Negate(expr) => Ok(-self.eval_expr(*expr)?),
            Expr::Binary { lhs, op, rhs } => self.eval_binary(*lhs, op, *rhs),
        }
    }

    /// Evaluates an identifier expression.
    fn eval_ident(&self, name: String) -> Result<f64, RuntimeError> {
        match self.variables.get(&name).copied() {
            None => Err(RuntimeError::UndefinedVariable(name)),
            Some(value) => Ok(value),
        }
    }

    /// Evaluates a binary expression.
    fn eval_binary(&mut self, lhs: Expr, op: BinOp, rhs: Expr) -> Result<f64, RuntimeError> {
        // Assignment is handled as a special case because its operands cannot
        // be eagerly evaluated. The left-hand-side is an assignment target, not
        // an actual expression. The right-hand-side should only be evaluated if
        // the target is a variable.
        if let BinOp::Assign = op {
            return self.eval_assignment(lhs, rhs);
        }

        let lhs = self.eval_expr(lhs)?;
        let rhs = self.eval_expr(rhs)?;

        Ok(match op {
            BinOp::Assign => unreachable!(),
            BinOp::Add => lhs + rhs,
            BinOp::Subtract => lhs - rhs,
            BinOp::Multiply => lhs * rhs,
            BinOp::Divide => lhs / rhs,
        })
    }

    /// Evaluates an assignment expression.
    fn eval_assignment(&mut self, target: Expr, source: Expr) -> Result<f64, RuntimeError> {
        if let Expr::Ident(name) = target {
            let value = self.eval_expr(source)?;
            self.variables.insert(name, value);
            Ok(value)
        } else {
            Err(RuntimeError::NonVariableAssignment)
        }
    }
}

/// An error encountered at runtime.
#[derive(Debug)]
pub enum RuntimeError {
    /// An undefined variable was evaluated.
    UndefinedVariable(String),

    /// A non-variable assignment target was assigned to.
    NonVariableAssignment,
}

impl error::Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UndefinedVariable(name) => write!(f, "variable '{name}' is undefined"),
            Self::NonVariableAssignment => write!(f, "assigned to a non-variable"),
        }
    }
}
