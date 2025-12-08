use std::fmt::{self, Display, Formatter};

use super::{Ast, BinOp, Expr, Literal, LogicOp, Stmt, UnOp};

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
            Self::Literal(literal) => literal.fmt(f),
            Self::Ident(name) => f.write_str(name),
            Self::Paren(expr) => write_s_expr(f, "p:", &[expr]),
            Self::Tuple(exprs) => write_s_expr(f, "t:", exprs),
            Self::Block(stmts) => write_s_expr(f, "b:", stmts),
            Self::Function(params, body) => {
                let mut args = params.iter().collect::<Vec<_>>();
                args.push(body);
                write_s_expr(f, "->", &args)
            }
            Self::Call(callee, args) => write_s_expr(f, callee, args),
            Self::Unary(op, expr) => write_s_expr(f, op, &[expr]),
            Self::Binary(op, lhs, rhs) => write_s_expr(f, op, &[lhs, rhs]),
            Self::Logic(op, lhs, rhs) => write_s_expr(f, op, &[lhs, rhs]),
            Self::Cond(cond, then, or) => write_s_expr(f, "?", &[cond, then, or]),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Bool(value) => value.fmt(f),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::Negate => "-",
            Self::Not => "!",
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
            Self::Power => "^",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Less => "<",
            Self::LessEqual => "<=",
            Self::Greater => ">",
            Self::GreaterEqual => ">=",
        };

        f.write_str(symbol)
    }
}

impl Display for LogicOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::And => "&&",
            Self::Or => "||",
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
