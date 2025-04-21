use crate::ast::{BinOp, Expr};

use super::{Parser, syntax_error::SyntaxError, token::Token};

impl Parser<'_> {
    /// Parses an infix expression.
    pub fn parse_infix(&mut self) -> Result<Expr, SyntaxError> {
        self.parse_infix_level(0)
    }

    /// Parses an infix expression with a minimum precedence.
    fn parse_infix_level(&mut self, min_precedence: u8) -> Result<Expr, SyntaxError> {
        let mut lhs = self.parse_atom()?;

        while let Some(op) = BinOp::from_token(self.peek()?) {
            let precedence = op.precedence().level();

            if precedence < min_precedence {
                break;
            }

            let min_precedence = match op.associativity() {
                Associativity::LeftToRight => precedence + 1,
                Associativity::RightToLeft => precedence,
            };

            self.next()?; // Skip operator token.
            let rhs = self.parse_infix_level(min_precedence)?;

            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }
}

impl BinOp {
    /// Returns an optional new binary operator from a token.
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Eq => Some(Self::Assign),
            Token::Plus => Some(Self::Add),
            Token::Minus => Some(Self::Sub),
            Token::Star => Some(Self::Mul),
            Token::Slash => Some(Self::Div),
            _ => None,
        }
    }

    /// Returns the binary operator's precedence.
    fn precedence(self) -> Precedence {
        match self {
            Self::Assign => Precedence::Assignment,
            Self::Add | Self::Sub => Precedence::Sum,
            Self::Mul | Self::Div => Precedence::Term,
        }
    }

    /// Returns the binary operator's associativity.
    fn associativity(self) -> Associativity {
        match self {
            Self::Assign => Associativity::RightToLeft,
            _ => Associativity::LeftToRight,
        }
    }
}

/// A binary operator's precedence level. The precedence levels *must* be
/// declared in order from lowest to highest.
#[derive(Clone, Copy)]
#[repr(u8)]
enum Precedence {
    /// The precedence of the assigment operator.
    Assignment,

    /// The precedence of the addition and subtraction operators.
    Sum,

    /// The precedence of the multiplication and division operators.
    Term,
}

impl Precedence {
    /// Returns the precedence level represented as an integer.
    fn level(self) -> u8 {
        self as u8
    }
}

/// A binary operator's associativity.
#[derive(Clone, Copy)]
enum Associativity {
    /// A left-to-right associativity.
    LeftToRight,

    /// A right-to-left associativity.
    RightToLeft,
}
