/// A lexical element of source code.
#[derive(Debug)]
pub enum Token {
    /// A number.
    #[cfg_attr(not(test), expect(dead_code, reason = "field should be debug printed"))]
    Number(f64),

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
