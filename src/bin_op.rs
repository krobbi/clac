use crate::token::Token;

/// A binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// An assignment operator.
    Assign,

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
            Self::Assign => Prec::Assignment,
            Self::Add | Self::Subtract => Prec::Sum,
            Self::Multiply | Self::Divide => Prec::Term,
        }
    }

    /// Returns the binary operator's associativity.
    pub fn assoc(self) -> Assoc {
        match self {
            Self::Assign => Assoc::Right,
            _ => Assoc::Left,
        }
    }
}

impl TryFrom<&Token> for BinOp {
    type Error = ();

    fn try_from(value: &Token) -> Result<Self, Self::Error> {
        match value {
            Token::Eq => Ok(Self::Assign),
            Token::Plus => Ok(Self::Add),
            Token::Minus => Ok(Self::Subtract),
            Token::Star => Ok(Self::Multiply),
            Token::Slash => Ok(Self::Divide),
            _ => Err(()),
        }
    }
}

/// A binary operator's precedence level. The precedence levels *must* be
/// declared in order from lowest to highest.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Prec {
    /// The precedence of the assigment operator.
    Assignment,

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
    Right,
}
