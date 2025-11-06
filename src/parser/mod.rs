#[cfg(test)]
mod tests;

mod infix;
mod lexer;
mod parse_error;

pub use self::parse_error::ParseError;

use std::mem;

use crate::ast::{Ast, Expr, Stmt, UnOp};

use self::lexer::{LexError, Lexer, Token, TokenType};

/// Parses an [`Ast`] from source code. This function returns a [`ParseError`]
/// if an [`Ast`] could not be parsed.
pub fn parse_source(source: &str) -> Result<Ast, ParseError> {
    let mut parser = Parser::try_new(source)?;
    parser.parse_ast()
}

/// A structure that parses an [`Ast`] from source code.
struct Parser<'a> {
    /// The [`Lexer`] for reading [`Token`]s from source code.
    lexer: Lexer<'a>,

    /// The next [`Token`].
    next_token: Token,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` from source code to be parsed. This function
    /// returns a [`LexError`] if a valid first [`Token`] could not be read.
    fn try_new(source: &'a str) -> Result<Self, LexError> {
        let mut lexer = Lexer::new(source);
        let next_token = lexer.bump()?;
        Ok(Self { lexer, next_token })
    }

    /// Parses an [`Ast`]. This function returns a [`ParseError`] if an [`Ast`]
    /// could not be parsed.
    fn parse_ast(&mut self) -> Result<Ast, ParseError> {
        let stmts = self.parse_sequence(TokenType::Eof)?;
        Ok(Ast(stmts))
    }

    /// Parses a sequence of [`Stmt`]s and consumes a [`Token`] that matches a
    /// terminator [`TokenType`]. This function returns a [`ParseError`] if a
    /// [`Stmt`] sequence could not be parsed.
    fn parse_sequence(&mut self, terminator: TokenType) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();

        while self.is_before_terminator(terminator) {
            let stmt = self.parse_stmt()?;
            self.eat(TokenType::Comma)?;
            stmts.push(stmt);
        }

        self.expect(terminator)?;
        Ok(stmts)
    }

    /// Parses a [`Stmt`]. This function returns a [`ParseError`] if a [`Stmt`]
    /// could not be parsed.
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let target = self.parse_expr()?;

        let stmt = if self.eat(TokenType::Eq)? {
            let source = self.parse_expr()?;

            if self.peek() == TokenType::Eq {
                return Err(ParseError::ChainedAssignment);
            }

            Stmt::Assign(target.into(), source.into())
        } else {
            Stmt::Expr(target.into())
        };

        Ok(stmt)
    }

    /// Parses a tuple of [`Expr`]s after consuming its opening parenthesis.
    /// This function returns a [`ParseError`] if a tuple could not be parsed.
    fn parse_tuple(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut exprs = Vec::new();

        while self.is_before_terminator(TokenType::CloseParen) {
            let expr = self.parse_expr()?;
            exprs.push(expr);

            if !self.eat(TokenType::Comma)? {
                break;
            }
        }

        self.expect(TokenType::CloseParen)?;
        Ok(exprs)
    }

    /// Parses an [`Expr`]. This function returns a [`ParseError`] if an
    /// [`Expr`] could not be parsed.
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_infix(0)
    }

    /// Parses an atom [`Expr`]. This function returns a [`ParseError`] if an
    /// atom [`Expr`] could not be parsed.
    fn parse_expr_atom(&mut self) -> Result<Expr, ParseError> {
        let mut callee = match self.bump()? {
            Token::Number(value) => Expr::Number(value),
            Token::Ident(name) => Expr::Ident(name),
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenType::CloseParen)?;
                Expr::Paren(expr.into())
            }
            Token::OpenBrace => {
                let stmts = self.parse_sequence(TokenType::CloseBrace)?;
                Expr::Block(stmts)
            }
            Token::Minus => {
                let rhs = self.parse_expr_atom()?;
                Expr::Unary(UnOp::Negate, rhs.into())
            }
            token => return Err(ParseError::ExpectedExpr(token)),
        };

        while self.eat(TokenType::OpenParen)? {
            let args = self.parse_tuple()?;
            callee = Expr::Call(callee.into(), args);
        }

        Ok(callee)
    }

    /// Returns the next [`Token`]'s [`TokenType`].
    fn peek(&self) -> TokenType {
        self.next_token.as_type()
    }

    /// Returns `true` if the next [`Token`] does not match a terminator
    /// [`TokenType`] and is before the end of source code.
    fn is_before_terminator(&self, terminator: TokenType) -> bool {
        let next_token_type = self.peek();
        next_token_type != terminator && next_token_type != TokenType::Eof
    }

    /// Consumes the next [`Token`]. This function returns a [`LexError`] if a
    /// valid following [`Token`] could not be read.
    fn bump(&mut self) -> Result<Token, LexError> {
        let following_token = self.lexer.bump()?;
        Ok(mem::replace(&mut self.next_token, following_token))
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns `true` if a [`Token`] was consumed and returns a
    /// [`LexError`] if a valid following [`Token`] could not be read.
    fn eat(&mut self, expected: TokenType) -> Result<bool, LexError> {
        let is_match = self.peek() == expected;

        if is_match {
            self.bump()?;
        }

        Ok(is_match)
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns a [`ParseError`] if the next [`Token`] does not
    /// match the expected [`TokenType`].
    fn expect(&mut self, expected: TokenType) -> Result<(), ParseError> {
        let actual = self.bump()?;

        if actual.as_type() == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(expected, actual))
        }
    }
}
