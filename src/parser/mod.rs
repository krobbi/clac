mod infix;
mod lexer;
mod syntax_error;
mod token;

use std::mem;

use lexer::Lexer;
use syntax_error::SyntaxError;
use token::Token;

use crate::ast::Expr;

/// Parses a program from source code.
pub fn parse_source(source: &str) -> Result<Vec<Expr>, SyntaxError> {
    Parser::new(source)?.parse_program()
}

/// A structure that generates expressions from source code.
struct Parser<'a> {
    /// The lexer.
    lexer: Lexer<'a>,

    /// The next token.
    next_token: Token,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from source code.
    fn new(source: &'a str) -> Result<Self, SyntaxError> {
        let mut lexer = Lexer::new(source);
        let next_token = lexer.scan_token()?;
        Ok(Self { lexer, next_token })
    }

    /// Parses a program.
    fn parse_program(&mut self) -> Result<Vec<Expr>, SyntaxError> {
        self.parse_sequence(Token::Eof)
    }

    /// Parses a sequence of expressions and consumes its terminator.
    fn parse_sequence(&mut self, terminator: Token) -> Result<Vec<Expr>, SyntaxError> {
        let mut exprs = vec![];

        if self.eat(&terminator)? {
            return Ok(exprs);
        }

        loop {
            exprs.push(self.parse_expr()?);

            if !self.eat(&Token::Comma)? {
                break;
            }
        }

        self.expect(terminator)?;
        Ok(exprs)
    }

    /// Parses an expression.
    fn parse_expr(&mut self) -> Result<Expr, SyntaxError> {
        self.parse_infix()
    }

    /// Parses an atom expression.
    fn parse_atom(&mut self) -> Result<Expr, SyntaxError> {
        let mut callee = match self.next()? {
            Token::Literal(literal) => Expr::Literal(literal),
            Token::Ident(name) => Expr::Ident(name),
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen)?;
                expr
            }
            Token::OpenBrace => Expr::Block(self.parse_sequence(Token::CloseBrace)?),
            Token::Minus => {
                let rhs = self.parse_atom()?;
                Expr::Negate(Box::new(rhs))
            }
            token => return Err(SyntaxError::ExpectedExpr(token)),
        };

        while self.eat(&Token::OpenParen)? {
            let args = self.parse_sequence(Token::CloseParen)?;

            callee = Expr::Call {
                callee: Box::new(callee),
                args,
            };
        }

        Ok(callee)
    }

    /// Returns the next token without consuming it.
    fn peek(&self) -> &Token {
        &self.next_token
    }

    /// Consumes the next token without returning it.
    fn bump(&mut self) -> Result<(), SyntaxError> {
        self.next_token = self.lexer.scan_token()?;
        Ok(())
    }

    /// Consumes and returns the next token.
    fn next(&mut self) -> Result<Token, SyntaxError> {
        Ok(mem::replace(&mut self.next_token, self.lexer.scan_token()?))
    }

    /// Returns whether the next token matches a token kind.
    fn check(&self, kind: &Token) -> bool {
        mem::discriminant(self.peek()) == mem::discriminant(kind)
    }

    /// Consumes the next token if it matches a token kind and returns whether
    /// it was consumed.
    fn eat(&mut self, kind: &Token) -> Result<bool, SyntaxError> {
        let is_match = self.check(kind);

        if is_match {
            self.bump()?;
        }

        Ok(is_match)
    }

    /// Consumes the next token with an expected token kind.
    fn expect(&mut self, expected: Token) -> Result<(), SyntaxError> {
        if self.eat(&expected)? {
            Ok(())
        } else {
            let actual = self.next()?;
            Err(SyntaxError::UnexpectedToken { expected, actual })
        }
    }
}
