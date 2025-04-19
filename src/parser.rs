use std::{error, fmt, iter, mem};

use crate::{
    bin_op::{self, BinOp},
    expr::Expr,
    lexer::{LexError, Lexer},
    token::Token,
};

/// Parses a program from source code.
pub fn parse_source(source: &str) -> Result<Vec<Expr>, ParseError> {
    Parser::new(source).parse_program()
}

/// A syntax error encountered while parsing.
#[derive(Debug)]
pub enum ParseError {
    /// An error caused by a lexing error.
    Lex(LexError),

    /// A token was encountered that does not match an expected token kind.
    Unexpected { expected: Token, actual: Token },

    /// A token was encountered that does not begin an expected expression.
    NonExpression(Token),
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Lex(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Lex(e) => e.fmt(f),
            Self::Unexpected { expected, actual } => write!(f, "expected {expected}, got {actual}"),
            Self::NonExpression(t) => write!(f, "expected an expression, got {t}"),
        }
    }
}

impl From<LexError> for ParseError {
    fn from(value: LexError) -> Self {
        Self::Lex(value)
    }
}

/// A structure that generates expressions from source code.
struct Parser<'a> {
    /// The lexer.
    lexer: iter::Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from source code.
    fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
        }
    }

    /// Parses a program.
    fn parse_program(&mut self) -> Result<Vec<Expr>, ParseError> {
        let program = self.parse_sequence(&Token::Eof)?;
        self.expect(Token::Eof)?;
        Ok(program)
    }

    /// Parses a sequence.
    fn parse_sequence(&mut self, terminator: &Token) -> Result<Vec<Expr>, ParseError> {
        let mut sequence = vec![];

        if self.check(terminator)? {
            return Ok(sequence);
        }

        loop {
            sequence.push(self.parse_expr()?);

            if !self.eat(&Token::Comma)? {
                break Ok(sequence);
            }
        }
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_infix(0)
    }

    /// Parses an infix expression.
    fn parse_infix(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_atom()?;

        while let Ok(op) = BinOp::try_from(self.peek()?) {
            let op_prec = op.prec().level();

            if op_prec < min_prec {
                break;
            }

            let min_prec = match op.assoc() {
                bin_op::Assoc::Left => op_prec + 1,
                bin_op::Assoc::Right => op_prec,
            };

            self.next()?; // Skip operator token.
            let rhs = self.parse_infix(min_prec)?;

            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    /// Parses an atom expression.
    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        match self.next()? {
            Token::Literal(value) => Ok(Expr::Literal(value)),
            Token::Ident(name) => Ok(Expr::Ident(name)),
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen)?;
                Ok(expr)
            }
            Token::Minus => {
                let rhs = self.parse_atom()?;
                Ok(Expr::Negate(Box::new(rhs)))
            }
            t => Err(ParseError::NonExpression(t)),
        }
    }

    /// Returns the next token without consuming it.
    fn peek(&mut self) -> Result<&Token, LexError> {
        match self.lexer.peek().unwrap() {
            Ok(t) => Ok(t),
            Err(e) => Err(e.clone()),
        }
    }

    /// Consumes and returns the next token.
    fn next(&mut self) -> Result<Token, LexError> {
        self.lexer.next().unwrap()
    }

    /// Returns whether the next token matches a token kind.
    fn check(&mut self, kind: &Token) -> Result<bool, LexError> {
        Ok(mem::discriminant(self.peek()?) == mem::discriminant(kind))
    }

    /// Consumes the next token if it matches a token kind and returns whether
    /// it was consumed.
    fn eat(&mut self, kind: &Token) -> Result<bool, LexError> {
        let is_match = self.check(kind)?;

        if is_match {
            self.next()?;
        }

        Ok(is_match)
    }

    /// Consumes the next token with an expected token kind.
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.eat(&expected)? {
            Ok(())
        } else {
            let actual = self.next()?;
            Err(ParseError::Unexpected { expected, actual })
        }
    }
}
