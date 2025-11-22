use crate::ast::{BinOp, Expr};

use super::{ParseError, Parser, lexer::TokenType};

impl Parser<'_> {
    /// Parses an infix [`Expr`] with a minimum precedence level. This function
    /// returns a [`ParseError`] if an infix [`Expr`] could not be parsed.
    pub fn parse_expr_infix(&mut self, min_precedence: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_expr_call()?;

        while let Some(op) = BinOp::from_token_type(self.peek()) {
            let precedence = op.precedence();

            if precedence < min_precedence {
                break;
            }

            let min_precedence = match op.associativity() {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
            };

            self.bump()?; // Consume the operator token.
            let rhs = self.parse_expr_infix(min_precedence)?;
            lhs = Expr::Binary(op, lhs.into(), rhs.into());
        }

        Ok(lhs)
    }
}

impl BinOp {
    /// Creates a new `BinOp` from a [`TokenType`]. This function returns
    /// [`None`] if the [`TokenType`] does not correspond to a `BinOp`.
    fn from_token_type(token_type: TokenType) -> Option<Self> {
        let op = match token_type {
            TokenType::Plus => Self::Add,
            TokenType::Minus => Self::Subtract,
            TokenType::Star => Self::Multiply,
            TokenType::Slash => Self::Divide,
            _ => return None,
        };

        Some(op)
    }

    /// Returns the `BinOp`'s precedence level.
    fn precedence(self) -> u8 {
        match self {
            Self::Add | Self::Subtract => 0,
            Self::Multiply | Self::Divide => 1,
        }
    }

    /// Returns the `BinOp`'s [`Associativity`].
    fn associativity(self) -> Associativity {
        #[expect(
            clippy::match_single_binding,
            reason = "no right-associative operators have been defined"
        )]
        match self {
            _ => Associativity::Left,
        }
    }
}

/// A [`BinOp`]'s associativity.
#[derive(Clone, Copy)]
enum Associativity {
    /// Left to right.
    Left,

    /// Right to left.
    #[expect(dead_code, reason = "no right-associative operators have been defined")]
    Right,
}
