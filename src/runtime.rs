use std::{collections::HashMap, error, fmt, ops};

use crate::{bin_op::BinOp, expr::Expr, value::Value};

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
        let value = self.eval_expr(expr)?;
        println!("{value}");
        Ok(())
    }

    /// Evaluates an expression.
    fn eval_expr(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(value) => Ok(value),
            Expr::Ident(name) => self.eval_ident(name),
            Expr::Negate(expr) => Ok(-self.eval_expr(*expr)?),
            Expr::Binary { lhs, op, rhs } => self.eval_binary(*lhs, op, *rhs),
        }
    }

    /// Evaluates an identifier expression.
    fn eval_ident(&self, name: String) -> Result<Value, RuntimeError> {
        match self.variables.get(&name).cloned() {
            None => Err(RuntimeError::UndefinedVariable(name)),
            Some(value) => Ok(value),
        }
    }

    /// Evaluates a binary expression.
    fn eval_binary(&mut self, lhs: Expr, op: BinOp, rhs: Expr) -> Result<Value, RuntimeError> {
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
            BinOp::Sub => lhs - rhs,
            BinOp::Mul => lhs * rhs,
            BinOp::Div => lhs / rhs,
        })
    }

    /// Evaluates an assignment expression.
    fn eval_assignment(&mut self, target: Expr, source: Expr) -> Result<Value, RuntimeError> {
        if let Expr::Ident(name) = target {
            let value = self.eval_expr(source)?;
            self.variables.insert(name, value.clone());
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

impl ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Number(rhs) => Self::Number(-rhs),
        }
    }
}

macro_rules! value_binop_impl {
    ($trait:path, $fn:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Self;

            fn $fn(self, rhs: Self) -> Self::Output {
                match (self, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs $op rhs),
                }
            }
        }
    }
}

value_binop_impl!(ops::Add, add, +);
value_binop_impl!(ops::Sub, sub, -);
value_binop_impl!(ops::Mul, mul, *);
value_binop_impl!(ops::Div, div, /);
