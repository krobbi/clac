use std::fmt::{self, Display, Formatter};

use crate::ir::Value;

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Function(_) | Self::Native(_) => f.write_str("function"),
        }
    }
}
