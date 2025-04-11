/// A syntactic element of source code.
#[derive(Debug)]
pub enum Token {
    /// A literal value.
    Literal(#[allow(dead_code, reason = "field is used in debug printing")] f64),

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
