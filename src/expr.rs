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
            Self::Paren(expr) => expr.fmt(f),
            Self::Negate(expr) => write!(f, "(-{expr})"),
            Self::Binary { lhs, op, rhs } => write!(f, "({lhs} {op} {rhs})"),
        }
    }
}

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

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => '+'.fmt(f),
            Self::Subtract => '-'.fmt(f),
            Self::Multiply => '*'.fmt(f),
            Self::Divide => '/'.fmt(f),
        }
    }
}

impl TryFrom<Token> for BinOp {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Add => Ok(Self::Add),
            Token::Subtract => Ok(Self::Subtract),
            Token::Multiply => Ok(Self::Multiply),
            Token::Divide => Ok(Self::Divide),
            _ => Err(()),
        }
    }
}
