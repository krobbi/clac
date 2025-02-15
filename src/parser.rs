use std::{error, fmt, mem};

use crate::{
    expr::{BinOp, Expr},
    lexer::{LexError, Lexer},
    token::Token,
};

/// Parses a statement from statement source code.
pub fn parse_source(source: &str) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(source)?;
    parser.parse_stmt()
}

/// A syntax error encountered while parsing.
#[derive(Debug)]
pub enum ParseError {
    /// An error caused by a lex error.
    Lex(LexError),

    /// A token was encountered that does not match an expected token.
    UnexpectedToken { expected: Token, actual: Token },

    /// A token was encountered that does not begin an expected expression.
    NonExpression(Token),
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Lex(error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lex(error) => error.fmt(f),
            Self::UnexpectedToken { expected, actual } => {
                write!(f, "expected {expected}, got {actual}")
            }
            Self::NonExpression(token) => write!(f, "expected an expression, got {token}"),
        }
    }
}

impl From<LexError> for ParseError {
    fn from(value: LexError) -> Self {
        Self::Lex(value)
    }
}

/// A structure that parses a statement from statement source code.
struct Parser<'a> {
    /// The lexer.
    lexer: Lexer<'a>,

    /// The next token.
    next: Token,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from statement source code.
    fn new(source: &'a str) -> Result<Self, LexError> {
        let mut lexer = Lexer::new(source);
        let next = lexer.next()?;
        Ok(Self { lexer, next })
    }

    /// Parses a statement.
    fn parse_stmt(&mut self) -> Result<Expr, ParseError> {
        let expr = self.parse_expr()?;
        self.expect(Token::End)?;
        Ok(expr)
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_term_expr()
    }

    /// Parses a term expression.
    fn parse_term_expr(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_negate_expr()?;

        while let Token::Multiply | Token::Divide = self.next {
            let op = BinOp::try_from(self.advance()?).unwrap();
            let rhs = self.parse_negate_expr()?;

            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(lhs)
    }

    /// Parses a negation expression.
    fn parse_negate_expr(&mut self) -> Result<Expr, ParseError> {
        if let Token::Subtract = self.next {
            self.advance()?;
            let expr = self.parse_negate_expr()?;
            Ok(Expr::Negate(Box::new(expr)))
        } else {
            self.parse_primary_expr()
        }
    }

    /// Parses a primary expression.
    fn parse_primary_expr(&mut self) -> Result<Expr, ParseError> {
        match self.advance()? {
            Token::Number(value) => Ok(Expr::Number(value)),
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen)?;
                Ok(Expr::Paren(Box::new(expr)))
            }
            token => Err(ParseError::NonExpression(token)),
        }
    }

    /// Consumes and returns the next token.
    fn advance(&mut self) -> Result<Token, LexError> {
        Ok(mem::replace(&mut self.next, self.lexer.next()?))
    }

    /// Consumes the next expected token.
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let actual = self.advance()?;

        if actual == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken { expected, actual })
        }
    }
}
