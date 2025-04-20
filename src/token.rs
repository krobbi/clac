use std::fmt;

use crate::value::Value;

/// A syntactic element of source code.
#[derive(Debug)]
pub enum Token {
    /// A literal value.
    Literal(Value),

    /// An identifier.
    Ident(String),

    /// An opening parenthesis `(`.
    OpenParen,

    /// A closing parenthesis `)`.
    CloseParen,

    /// A comma `,`.
    Comma,

    /// An equals sign `=`.
    Eq,

    /// A plus sign `+`.
    Plus,

    /// A minus sign `-`.
    Minus,

    /// An asterisk `*`.
    Star,

    /// A slash `/`.
    Slash,

    /// An end of source code marker.
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(Value::Void) => unreachable!(),
            Self::Literal(value @ Value::Number(_)) => write!(f, "number '{value}'"),
            Self::Ident(name) => write!(f, "identifier '{name}'"),
            Self::OpenParen => write!(f, "opening '('"),
            Self::CloseParen => write!(f, "closing ')'"),
            Self::Comma => write!(f, "','"),
            Self::Eq => write!(f, "'='"),
            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Star => write!(f, "'*'"),
            Self::Slash => write!(f, "'/'"),
            Self::Eof => write!(f, "end-of-file"),
        }
    }
}
