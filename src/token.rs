use std::fmt;

/// A syntactic element of source code.
#[derive(Debug)]
pub enum Token {
    /// A literal value.
    Literal(f64),

    /// An opening parenthesis `(`.
    OpenParen,

    /// A closing parenthesis `)`.
    CloseParen,

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
            Self::Literal(value) => write!(f, "literal '{value}'"),
            Self::OpenParen => write!(f, "opening '('"),
            Self::CloseParen => write!(f, "closing ')'"),
            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Star => write!(f, "'*'"),
            Self::Slash => write!(f, "'/'"),
            Self::Eof => write!(f, "end-of-file"),
        }
    }
}
