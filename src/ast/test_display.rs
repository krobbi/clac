use std::fmt::{self, Display, Formatter};

use super::*;

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write_s_expr(f, "a:", &self.0)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(target, source) => write_s_expr(f, "=", &[target, source]),
            Self::Expr(expr) => expr.fmt(f),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Ident(name) => f.write_str(name),
            Self::Paren(expr) => write_s_expr(f, "p:", &[expr]),
            Self::Tuple(exprs) => write_s_expr(f, "t:", exprs),
            Self::Block(stmts) => write_s_expr(f, "b:", stmts),
            Self::Call(callee, args) => write_s_expr(f, callee, args),
            Self::Unary(op, expr) => write_s_expr(f, op, &[expr]),
            Self::Binary(op, lhs, rhs) => write_s_expr(f, op, &[lhs, rhs]),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Negate => "-",
        };

        f.write_str(symbol)
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
        };

        f.write_str(symbol)
    }
}

/// Writes an operator and arguments to a [`Formatter`] as an S-expression. This
/// function returns a [`fmt::Error`] if the S-expression could not be written.
fn write_s_expr(f: &mut Formatter<'_>, op: impl Display, args: &[impl Display]) -> fmt::Result {
    write!(f, "({op}")?;

    for arg in args {
        write!(f, " {arg}")?;
    }

    f.write_str(")")
}
