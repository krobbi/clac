/// A lexical element of source code.
#[derive(Debug)]
pub enum Token {
    /// An opening parenthesis.
    OpenParen,

    /// A closing parenthesis.
    CloseParen,

    /// A comma.
    Comma,

    /// A plus sign.
    Plus,

    /// A minus sign.
    Minus,

    /// An asterisk.
    Star,

    /// A forward slash.
    Slash,

    /// An end of source code marker.
    Eof,
}
