use std::fmt::{self, Display, Formatter};

use super::{Ast, BinOp, Expr, Stmt, UnOp};

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("(p:")?;

        for stmt in &self.0 {
            write!(f, " {stmt}")?;
        }

        f.write_str(")")
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(target, source) => write!(f, "(= {target} {source})"),
            Self::Expr(expr) => expr.fmt(f),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Ident(name) => f.write_str(name),
            Self::Paren(expr) => write!(f, "(i: {expr})"),
            Self::Call(callee, args) => {
                write!(f, "({callee}")?;

                for arg in args {
                    write!(f, " {arg}")?;
                }

                f.write_str(")")
            }
            Self::Unary(op, expr) => write!(f, "({op} {expr})"),
            Self::Binary(op, lhs, rhs) => write!(f, "({op} {lhs} {rhs})"),
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
