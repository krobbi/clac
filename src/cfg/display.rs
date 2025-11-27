use std::fmt::{self, Display, Formatter, Write as _};

use super::{Block, Cfg, Exit, Label};

impl Display for Cfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for (label, block) in self.blocks.iter().enumerate().map(|(i, b)| (Label(i), b)) {
            let _ = writeln!(buffer, "{label}:\n{:8}{block}\n", "");
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
        self.exit.fmt(f)
    }
}

impl Display for Exit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => f.write_str("halt"),
        }
    }
}
