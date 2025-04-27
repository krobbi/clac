use std::{error, fmt};

use super::token::Token;

/// A syntax error encountered while parsing.
#[derive(Debug)]
pub enum SyntaxError {
    /// An character was encountered that does not begin a token.
    UnexpectedChar(char),

    /// A token was encountered that does not match an expected token kind.
    UnexpectedToken { expected: Token, actual: Token },

    /// A token was encountered that does not begin an expected expression.
    ExpectedExpr(Token),
}

impl error::Error for SyntaxError {}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "unexpected character '{}'", c.escape_default()),
            Self::UnexpectedToken { expected, actual } => {
                write!(f, "expected {expected}, got {actual}")
            }
            Self::ExpectedExpr(token) => write!(f, "expected an expression, got {token}"),
        }
    }
}
