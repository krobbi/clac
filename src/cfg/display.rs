use std::fmt::{self, Display, Formatter, Write as _};

use super::{BasicBlock, Cfg, Instruction, Label, Terminator};

impl Display for Cfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for (label, basic_block) in self
            .basic_blocks
            .iter()
            .enumerate()
            .map(|(i, b)| (Label(i), b))
        {
            let _ = writeln!(buffer, "{label}:");

            for line in basic_block.to_string().lines() {
                let _ = writeln!(buffer, "{:8}{line}", "");
            }
        }

        f.write_str(buffer.trim_end())
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => f.write_str("main"),
            index => write!(f, ".L{index}"),
        }
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for instruction in &self.instructions {
            let _ = writeln!(buffer, "{instruction}");
        }

        let _ = write!(buffer, "{}", self.terminator);
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
            Self::Power => f.write_str("power"),
            Self::Equal => f.write_str("equal"),
            Self::NotEqual => f.write_str("not_equal"),
            Self::Less => f.write_str("less"),
            Self::LessEqual => f.write_str("less_equal"),
            Self::Greater => f.write_str("greater"),
            Self::GreaterEqual => f.write_str("greater_equal"),
            Self::LoadLocal(offset) => write!(f, "{:16}[{offset}]", "load_local"),
            Self::StoreLocal(offset) => write!(f, "{:16}[{offset}]", "store_local"),
            Self::LoadGlobal(symbol) => write!(f, "{:16}{symbol}", "load_global"),
            Self::StoreGlobal(symbol) => write!(f, "{:16}{symbol}", "store_global"),
            Self::DefineUpvalue => f.write_str("define_upvalue"),
            Self::LoadUpvalue(offset) => write!(f, "{:16}[{offset}]", "load_upvalue"),
            Self::DropUpvalues(count) => write!(f, "{:16}{count}", "drop_upvalues"),
            Self::IntoClosure => f.write_str("into_closure"),
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => f.write_str("halt"),
            Self::Jump(label) => write!(f, "{:16}{label}", "jump"),
            Self::Branch(then_label, else_label) => {
                write!(f, "{:16}{then_label} else {else_label}", "branch")
            }
            Self::Call(arity, label) => write!(f, "{:16}({arity}) return {label}", "call"),
            Self::Return => f.write_str("return"),
        }
    }
}
