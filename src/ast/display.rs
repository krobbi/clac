use std::fmt::{self, Display, Formatter};

use super::{Ast, BinOp, Expr, Literal, LogicOp, UnOp};

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_s_expr(f, "a:", &self.0)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Variable(symbol) => write!(f, "{symbol}"),
            Self::Paren(expr) => fmt_s_expr(f, "p:", &[expr]),
            Self::Tuple(exprs) => fmt_s_expr(f, "t:", exprs),
            Self::Block(stmts) => fmt_s_expr(f, "b:", stmts),
            Self::Assign(target, source) => fmt_s_expr(f, "=", &[target, source]),
            Self::Function(list, body) => fmt_s_expr(f, "->", &[list, body]),
            Self::Call(callee, list) => fmt_s_expr(f, callee, &[list]),
            Self::Unary(op, expr) => fmt_s_expr(f, op, &[expr]),
            Self::Binary(op, lhs, rhs) => fmt_s_expr(f, op, &[lhs, rhs]),
            Self::Logic(op, lhs, rhs) => fmt_s_expr(f, op, &[lhs, rhs]),
            Self::Cond(cond, then_expr, else_expr) => {
                fmt_s_expr(f, "?", &[cond, then_expr, else_expr])
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{value}"),
            Self::Bool(value) => write!(f, "{value}"),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            Self::Negate => "-",
            Self::Not => "!",
        };

        write!(f, "{op}")
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Power => "^",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
        };

        write!(f, "{op}")
    }
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            Self::And => "&&",
            Self::Or => "||",
        };

        write!(f, "{op}")
    }
}

/// Formats an operator and arguments as an S-expression with a [`Formatter`].
/// This function returns a [`fmt::Error`] if an error occurred.
fn fmt_s_expr<O: Display, A: Display>(f: &mut Formatter<'_>, op: O, args: &[A]) -> fmt::Result {
    write!(f, "({op}")?;

    for arg in args {
        write!(f, " {arg}")?;
    }

    write!(f, ")")
}
