use std::fmt::{self, Display, Formatter};

use super::{Ast, BinOp, Expr, Literal, LogicOp, UnOp};

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_s_expr("a:", &self.0, f)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => Display::fmt(literal, f),
            Self::Ident(symbol) => Display::fmt(symbol, f),
            Self::Paren(expr) => fmt_s_expr("p:", &[expr], f),
            Self::Tuple(exprs) => fmt_s_expr("t:", exprs, f),
            Self::Block(stmts) => fmt_s_expr("b:", stmts, f),
            Self::Assign(target, source) => fmt_s_expr("=", &[target, source], f),
            Self::Function(params, body) => {
                let mut args = params.iter().collect::<Vec<_>>();
                args.push(body);
                fmt_s_expr("->", &args, f)
            }
            Self::Call(callee, args) => fmt_s_expr(callee, args, f),
            Self::Unary(op, expr) => fmt_s_expr(op, &[expr], f),
            Self::Binary(op, lhs, rhs) => fmt_s_expr(op, &[lhs, rhs], f),
            Self::Logic(op, lhs, rhs) => fmt_s_expr(op, &[lhs, rhs], f),
            Self::Cond(cond, then_expr, else_expr) => {
                fmt_s_expr("?", &[cond, then_expr, else_expr], f)
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => Display::fmt(value, f),
            Self::Bool(value) => Display::fmt(value, f),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            Self::Negate => "-",
            Self::Not => "!",
        };

        f.write_str(op)
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

        f.write_str(op)
    }
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            Self::And => "&&",
            Self::Or => "||",
        };

        f.write_str(op)
    }
}

/// Formats an operator and arguments as an S-expression with a [`Formatter`].
/// This function returns a [`fmt::Error`] if an error occurred.
fn fmt_s_expr<O: Display, A: Display>(op: O, args: &[A], f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({op}")?;

    for arg in args {
        write!(f, " {arg}")?;
    }

    f.write_str(")")
}
