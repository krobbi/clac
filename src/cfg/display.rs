use std::fmt::{self, Display, Formatter, Write as _};

use super::{Block, Cfg, Exit, Instruction, Label};

// TODO: Add support for functions to CFG dumping.
impl Display for Cfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for (label, block) in self.blocks.iter().enumerate().map(|(i, b)| (Label(i), b)) {
            let _ = writeln!(buffer, "{label}:");

            for line in block.to_string().lines() {
                let _ = writeln!(buffer, "{:8}{line}", "");
            }

            let _ = writeln!(buffer);
        }

        f.write_str(buffer.trim_end())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => f.write_str(".main"),
            index => write!(f, ".label_{index}"),
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for instruction in &self.instructions {
            let _ = writeln!(buffer, "{instruction}");
        }

        let _ = write!(buffer, "{}", self.exit);
        f.write_str(&buffer)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::PushLiteral(literal) => write!(f, "{:16}{literal}", "push_literal"),
            Self::PushFunction(_) => write!(f, "{:16}...", "push_function"),
            Self::Drop(count) => write!(f, "{:16}{count}", "drop"),
            Self::Print => f.write_str("print"),
            Self::Negate => f.write_str("negate"),
            Self::Not => f.write_str("not"),
            Self::Add => f.write_str("add"),
            Self::Subtract => f.write_str("subtract"),
            Self::Multiply => f.write_str("multiply"),
            Self::Divide => f.write_str("divide"),
            Self::LoadLocal(offset) => write!(f, "{:16}[{offset}]", "load_local"),
            Self::StoreLocal(offset) => write!(f, "{:16}[{offset}]", "store_local"),
            Self::LoadGlobal(name) => write!(f, "{:16}{name}", "load_global"),
            Self::StoreGlobal(name) => write!(f, "{:16}{name}", "store_global"),
            Self::DefineUpvalue => f.write_str("define_upvalue"),
            Self::LoadUpvalue(offset) => write!(f, "{:16}[{offset}]", "load_upvalue"),
            Self::DropUpvalues(count) => write!(f, "{:16}{count}", "drop_upvalues"),
            Self::IntoClosure => f.write_str("into_closure"),
        }
    }
}

impl Display for Exit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => f.write_str("halt"),
            Self::Call(arity, label) => write!(f, "{:16}({arity}) return {label}", "call"),
            Self::Return => f.write_str("return"),
        }
    }
}
