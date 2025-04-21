mod builtins;
mod runtime_error;
mod value;

use std::collections::HashMap;

use runtime_error::RuntimeError;
use value::Value;

use crate::ast::{BinOp, Expr};

/// The result of evaluating a voidable expression.
type EvalResult = Result<Option<Value>, RuntimeError>;

/// The runtime environment of a Clac program.
pub struct Runtime {
    /// The scope stack.
    scopes: Vec<HashMap<String, Value>>,
}

impl Runtime {
    /// Creates a new runtime environment.
    pub fn new() -> Self {
        let mut global_scope = HashMap::new();
        global_scope.insert("sqrt".to_string(), Value::Builtin(builtins::builtin_sqrt));

        Self {
            scopes: vec![global_scope],
        }
    }

    /// Executes a top-level expression.
    pub fn execute_expr(&mut self, expr: Expr) -> Result<(), RuntimeError> {
        if let Some(value) = self.eval_voidable(expr)? {
            println!("{value}");
        }

        Ok(())
    }

    /// Pushes a new innermost scope to the scope stack.
    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pops the innermost scope from the scope stack.
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Evaluates a voidable expression.
    fn eval_voidable(&mut self, expr: Expr) -> EvalResult {
        match expr {
            Expr::Literal(literal) => Ok(Some(literal.into())),
            Expr::Ident(name) => self.eval_ident(name),
            Expr::Block(exprs) => self.eval_block(exprs),
            Expr::Call { callee, args } => self.eval_call(*callee, args),
            Expr::Negate(rhs) => (-self.eval_value(*rhs)?).map(Some),
            Expr::Binary { lhs, op, rhs } => self.eval_binary(*lhs, op, *rhs),
        }
    }

    /// Evaluates a non-voidable expression.
    fn eval_value(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        match self.eval_voidable(expr) {
            Ok(None) => Err(RuntimeError::VoidValue),
            Ok(Some(value)) => Ok(value),
            Err(error) => Err(error),
        }
    }

    /// Evaluates an identifier expression.
    fn eval_ident(&self, name: String) -> EvalResult {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(&name) {
                return Ok(Some(value.clone()));
            }
        }

        Err(RuntimeError::UndefinedVariable(name))
    }

    /// Evaluates a block expression.
    fn eval_block(&mut self, mut exprs: Vec<Expr>) -> EvalResult {
        match exprs.pop() {
            None => Ok(None),
            Some(last_expr) => {
                self.push_scope();

                for expr in exprs {
                    if let Err(error) = self.eval_voidable(expr) {
                        self.pop_scope();
                        return Err(error);
                    }
                }

                let result = self.eval_voidable(last_expr);
                self.pop_scope();
                result
            }
        }
    }

    /// Evaluates a call expression.
    fn eval_call(&mut self, callee: Expr, args: Vec<Expr>) -> EvalResult {
        let callee = self.eval_value(callee)?;
        let mut arg_values = Vec::with_capacity(args.len());

        for arg in args {
            arg_values.push(self.eval_value(arg)?);
        }

        match callee {
            Value::Number(_) => Err(RuntimeError::NonFunctionCall),
            Value::Builtin(function) => function(&arg_values),
        }
    }

    /// Evaluates a binary expression.
    fn eval_binary(&mut self, lhs: Expr, op: BinOp, rhs: Expr) -> EvalResult {
        // Assignment is handled as a special case because its operands cannot
        // be eagerly evaluated. The left-hand-side is an assignment target, not
        // an actual expression. The right-hand-side is only evaluated after the
        // target is validated as a variable.
        if let BinOp::Assign = op {
            return self.eval_assignment(lhs, rhs);
        }

        let lhs = self.eval_value(lhs)?;
        let rhs = self.eval_value(rhs)?;

        match op {
            BinOp::Assign => unreachable!(),
            BinOp::Add => lhs + rhs,
            BinOp::Sub => lhs - rhs,
            BinOp::Mul => lhs * rhs,
            BinOp::Div => lhs / rhs,
        }
        .map(Some)
    }

    /// Evaluates an assignment expression.
    fn eval_assignment(&mut self, target: Expr, source: Expr) -> EvalResult {
        if let Expr::Ident(name) = target {
            let value = self.eval_value(source)?;

            for scope in self.scopes.iter_mut().rev() {
                if let Some(variable) = scope.get_mut(&name) {
                    *variable = value;
                    return Ok(None);
                }
            }

            self.scopes.last_mut().unwrap().insert(name, value);
            Ok(None)
        } else {
            Err(RuntimeError::NonVariableAssignment)
        }
    }
}
