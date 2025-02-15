use std::fmt;

use crate::token::Token;

/// A mathematical expression.
pub enum Expr {
    /// A number expression.
    Number(f64),

    /// A parenthesized expression.
    Paren(Box<Expr>),

    /// A negation expression.
    Negate(Box<Expr>),

    /// A binary expression.
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(value) => value.fmt(f),
            Self::Paren(expr) => write!(f, "({expr})"),
            Self::Negate(expr) => write!(f, "(-{expr})"),
            Self::Binary { lhs, op, rhs } => write!(f, "({lhs}{op}{rhs})"),
        }
    }
}

/// A binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// A multiplication operator.
    Multiply,

    /// A division operator.
    Divide,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Multiply => '*'.fmt(f),
            Self::Divide => '/'.fmt(f),
        }
    }
}

impl TryFrom<Token> for BinOp {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Multiply => Ok(Self::Multiply),
            Token::Divide => Ok(Self::Divide),
            _ => Err(()),
        }
    }
}
