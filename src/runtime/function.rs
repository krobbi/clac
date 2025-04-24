use std::slice;

use crate::ast::Expr;

/// A user-defined function.
pub struct Function {
    /// The parameter names.
    params: Vec<String>,

    /// The body expression.
    body: Expr,
}

impl Function {
    /// Creates a new function from its parameter names and body expression.
    pub fn new(params: Vec<String>, body: Expr) -> Self {
        Self { params, body }
    }

    /// Returns the number of arguments expected by the function.
    pub fn arity(&self) -> usize {
        self.params.len()
    }

    /// Returns an iterator over the function's parameter names.
    pub fn params(&self) -> slice::Iter<String> {
        self.params.iter()
    }

    /// Returns a reference to the function's body expression.
    pub fn body(&self) -> &Expr {
        &self.body
    }
}
