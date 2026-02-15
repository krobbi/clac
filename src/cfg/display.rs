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
        let name = match self {
            Self::PushLiteral(literal) => return write!(f, "{:16}{literal}", "push_literal"),
            Self::PushFunction(_) => return write!(f, "{:16}...", "push_function"),
            Self::PushGlobal(symbol) => return write!(f, "{:16}{symbol}", "push_global"),
            Self::PushLocal(offset) => return write!(f, "{:16}[{offset}]", "push_local"),
            Self::PushUpvar(offset) => return write!(f, "{:16}[{offset}]", "push_upvar"),
            Self::Pop(count) => return write!(f, "{:16}({count})", "pop"),
            Self::Print => "print",
            Self::Negate => "negate",
            Self::Not => "not",
            Self::Add => "add",
            Self::Subtract => "subtract",
            Self::Multiply => "multiply",
            Self::Divide => "divide",
            Self::Power => "power",
            Self::Equal => "equal",
            Self::NotEqual => "not_equal",
            Self::Less => "less",
            Self::LessEqual => "less_equal",
            Self::Greater => "greater",
            Self::GreaterEqual => "greater_equal",
            Self::StoreGlobal(symbol) => return write!(f, "{:16}{symbol}", "store_global"),
            Self::StoreLocal(offset) => return write!(f, "{:16}[{offset}]", "store_local"),
            Self::DefineUpvar => "define_upvar",
            Self::PopUpvars(count) => return write!(f, "{:16}({count})", "pop_upvars"),
            Self::IntoClosure => "into_closure",
        };

        f.write_str(name)
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
