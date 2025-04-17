use std::fmt;

use crate::token::Token;

/// A binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// An addition operator.
    Add,

    /// A subtraction operator.
    Subtract,

    /// A multiplication operator.
    Multiply,

    /// A division operator.
    Divide,
}

impl BinOp {
    /// Returns the binary operator's precedence.
    pub fn prec(self) -> Prec {
        match self {
            Self::Add | Self::Subtract => Prec::Sum,
            Self::Multiply | Self::Divide => Prec::Term,
        }
    }

    /// Returns the binary operator's associativity.
    #[allow(
        clippy::unused_self,
        reason = "self argument will be necessary in a future version"
    )]
    pub fn assoc(self) -> Assoc {
        // All binary operators are currently left-associative.
        // TODO: Add a condition here and remove the `#[allow]` attribute from
        // `Assoc` when right-associative operators are added.
        Assoc::Left
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
        }
    }
}

impl TryFrom<&Token> for BinOp {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Add),
            Token::Minus => Ok(Self::Subtract),
            Token::Star => Ok(Self::Multiply),
            Token::Slash => Ok(Self::Divide),
            _ => Err(()),
        }
    }
}

/// A binary operator's precedence level.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Prec {
    /// The precedence of the addition and subtraction operators.
    Sum,

    /// The precedence of the multiplication and division operators.
    Term,
}

impl Prec {
    /// Return the precedence level represented as an integer.
    pub fn level(self) -> u8 {
        self as u8
    }
}

/// A binary operator's associativity.
#[derive(Clone, Copy)]
pub enum Assoc {
    /// A left-to-right associativity.
    Left,

    /// A right-to-left associativity.
    #[allow(
        dead_code,
        reason = "no right-associative operators have been added yet"
    )]
    Right,
}
