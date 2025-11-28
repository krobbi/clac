use std::fmt::{self, Display, Formatter, Write as _};

use super::{Block, Cfg, Exit, Instruction, Label, UpvalueId};

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
            Self::PushFunction(label, arity) => write!(f, "{:16}{label}({arity})", "push_function"),
            Self::PushLocal(offset) => write!(f, "{:16}[{offset}]", "push_local"),
            Self::PushUpvalue(id) => write!(f, "{:16}{id}", "push_upvalue"),
            Self::PushGlobal(name) => write!(f, "{:16}{name}", "push_global"),
            Self::Drop => f.write_str("drop"),
            Self::Print => f.write_str("print"),
            Self::DefineUpvalue(id) => write!(f, "{:16}{id}", "define_upvalue"),
            Self::StoreLocal(offset) => write!(f, "{:16}[{offset}]", "store_local"),
            Self::StoreGlobal(name) => write!(f, "{:16}{name}", "store_global"),
            Self::IntoClosure => f.write_str("into_closure"),
            Self::Binary(op) => write!(f, "{:16}{op}", "binary"),
        }
    }
}

impl Display for UpvalueId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "upvalue_{}", self.0)
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
