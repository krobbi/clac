use crate::ast::{BinOp, Expr};

use super::{ParseError, Parser, lexer::TokenType};

impl Parser<'_> {
    // NOTE: This function parses a binary infix expression, so ideally the
    // infix parser should be used instead. However, the infix parser does not
    // support non-associative operators. To support non-associative
    // comparisons, the infix parser ignores comparison token types. Consider
    // adding support for more features in the infix parser for better
    // maintainability.
    /// Parses a comparison [`Expr`]. This function returns a [`ParseError`] if
    /// a comparison [`Expr`] could not be parsed.
    pub fn parse_expr_comparison(&mut self) -> Result<Expr, ParseError> {
        let lhs = self.parse_expr_infix()?;

        let comparison = if let Some(op) = BinOp::comparison_from_token_type(self.peek()) {
            self.bump()?; // Consume the operator token.
            let rhs = self.parse_expr_infix()?;

            if BinOp::comparison_from_token_type(self.peek()).is_some() {
                return Err(ParseError::ChainedComparison);
            }

            Expr::Binary(op, lhs.into(), rhs.into())
        } else {
            lhs
        };

        Ok(comparison)
    }
}

impl BinOp {
    /// Creates a new comparison `BinOp` from a [`TokenType`]. This function
    /// returns [`None`] if the [`TokenType`] does not correspond to a
    /// comparison `BinOp`.
    fn comparison_from_token_type(token_type: TokenType) -> Option<Self> {
        let op = match token_type {
            TokenType::EqEq => BinOp::Equal,
            TokenType::BangEq => BinOp::NotEqual,
            _ => return None,
        };

        Some(op)
    }
}
