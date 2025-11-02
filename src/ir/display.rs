use std::fmt::{self, Display, Formatter, Write as _};

use super::{BinOp, Body, Instruction, Ir, UnOp, Value};

impl Display for Ir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = "main:\n".to_owned();

        for line in self.0.to_string().lines() {
            let _ = writeln!(lines, "{:8}{line}", "");
        }

        f.write_str(lines.trim_end())
    }
}

impl Display for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines = String::new();

        for instruction in &self.0 {
            let _ = writeln!(lines, "{instruction}");
        }

        f.write_str(lines.trim_end())
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Push(value) => write!(f, "{:8}{value}", "push"),
            Self::PushGlobal(name) => write!(f, "{:8}{name}", "push"),
            Self::PushLocal(index) => write!(f, "{:8}[{index}]", "push"),
            Self::StoreGlobal(name) => write!(f, "{:8}{name}", "store"),
            Self::StoreLocal(index) => write!(f, "{:8}[{index}]", "store"),
            Self::Pop => f.write_str("pop"),
            Self::Print => f.write_str("print"),
            Self::Unary(op) => write!(f, "{:8}{op}", "unary"),
            Self::Binary(op) => write!(f, "{:8}{op}", "binary"),
            Self::AssertNonVoid => f.write_str("nonvoid"),
            Self::Halt => f.write_str("halt"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Void => f.write_str("{}"),
            Self::Number(value) => value.fmt(f),
        }
    }
}

impl Display for UnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Negate => "negate",
        };

        f.write_str(name)
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Add => "add",
            Self::Subtract => "subtract",
            Self::Multiply => "multiply",
            Self::Divide => "divide",
        };

        f.write_str(name)
    }
}
