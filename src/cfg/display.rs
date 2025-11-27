use std::fmt::{self, Display, Formatter, Write as _};

use super::{Block, Cfg, Exit, Instruction, Label};

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
            0 => f.write_str("main"),
            index => write!(f, "label_{index}"),
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
            Self::Drop => f.write_str("drop"),
        }
    }
}

impl Display for Exit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Halt => "halt",
            Self::Return => "return",
        };

        f.write_str(name)
    }
}
