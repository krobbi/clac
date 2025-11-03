use std::fmt::{self, Display, Formatter, Write as _};

use super::{BinOp, Body, Instruction, Ir, Value};

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
            Self::Drop => f.write_str("drop"),
            Self::Print => f.write_str("print"),
            Self::LoadLocal(index) => write!(f, "{:8}[{index}]", "load"),
            Self::LoadGlobal(name) => write!(f, "{:8}{name}", "load"),
            Self::StoreLocal(index) => write!(f, "{:8}[{index}]", "store"),
            Self::StoreGlobal(name) => write!(f, "{:8}{name}", "store"),
            Self::Binary(op) => write!(f, "{:8}{op}", "binary"),
            Self::Return => f.write_str("return"),
            Self::Halt => f.write_str("halt"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Function(_) => f.write_str("function"),
        }
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
