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

    /// An opening brace `{`.
    OpenBrace,

    /// A closing brace `}`.
    CloseBrace,

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
            Self::OpenParen => f.write_str("'('"),
            Self::CloseParen => f.write_str("')'"),
            Self::OpenBrace => f.write_str("'{'"),
            Self::CloseBrace => f.write_str("'}'"),
            Self::Comma => f.write_str("','"),
            Self::Eq => f.write_str("'='"),
            Self::Plus => f.write_str("'+'"),
            Self::Minus => f.write_str("'-'"),
            Self::Star => f.write_str("'*'"),
            Self::Slash => f.write_str("'/'"),
            Self::Eof => f.write_str("end of file"),
        }
    }
}
