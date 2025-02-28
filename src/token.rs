use std::fmt;

/// A syntactic element of statement source code.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// A number literal.
    Number(f64),

    /// An opening parenthesis `(`.
    OpenParen,

    /// A closing parenthesis `)`.
    CloseParen,

    /// An addition symbol `+`.
    Add,

    /// A subtraction symbol `-`.
    Subtract,

    /// A multiplication symbol `*`.
    Multiply,

    /// A division symbol `/`.
    Divide,

    /// An end-of-statement marker.
    End,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "number '{value}'"),
            Self::OpenParen => "opening '('".fmt(f),
            Self::CloseParen => "closing ')'".fmt(f),
            Self::Add => "'+'".fmt(f),
            Self::Subtract => "'-'".fmt(f),
            Self::Multiply => "'*'".fmt(f),
            Self::Divide => "'/'".fmt(f),
            Self::End => "end of statement".fmt(f),
        }
    }
}
