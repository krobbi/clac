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
pub fn parse_program(source: &str) -> Result<Vec<Expr>, Vec<SyntaxError>> {
    let mut errors = vec![];
    let program = Parser::new(source, &mut errors).parse_program();

    if errors.is_empty() {
        Ok(program)
    } else {
        Err(errors)
    }
}

/// A structure that generates expressions from source code.
struct Parser<'a> {
    /// The lexer.
    lexer: Lexer<'a>,

    /// The next token.
    next_token: Token,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from source code and a syntax error log.
    fn new(source: &'a str, errors: &'a mut Vec<SyntaxError>) -> Self {
        let mut lexer = Lexer::new(source, errors);
        let next_token = lexer.scan_token();
        Self { lexer, next_token }
    }

    /// Parses a program.
    fn parse_program(&mut self) -> Vec<Expr> {
        // TODO: Add error recovery to parser.
        match self.parse_sequence(Token::Eof) {
            Ok(program) => program,
            Err(error) => {
                self.emit_error(error);
                vec![]
            }
        }
    }

    /// Parses a sequence of expressions and consumes its terminator.
    fn parse_sequence(&mut self, terminator: Token) -> Result<Vec<Expr>, SyntaxError> {
        let mut exprs = vec![];

        if self.eat(&terminator) {
            return Ok(exprs);
        }

        loop {
            exprs.push(self.parse_expr()?);

            if !self.eat(&Token::Comma) {
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
        let mut callee = match self.next() {
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

        while self.eat(&Token::OpenParen) {
            let args = self.parse_sequence(Token::CloseParen)?;

            callee = Expr::Call {
                callee: Box::new(callee),
                args,
            };
        }

        Ok(callee)
    }

    /// Emits a syntax error to the syntax error log.
    fn emit_error(&mut self, error: SyntaxError) {
        self.lexer.emit_error(error);
    }

    /// Returns the next token without consuming it.
    fn peek(&self) -> &Token {
        &self.next_token
    }

    /// Consumes the next token without returning it.
    fn bump(&mut self) {
        self.next_token = self.lexer.scan_token();
    }

    /// Consumes and returns the next token.
    fn next(&mut self) -> Token {
        mem::replace(&mut self.next_token, self.lexer.scan_token())
    }

    /// Returns whether the next token matches a token kind.
    fn check(&self, kind: &Token) -> bool {
        mem::discriminant(self.peek()) == mem::discriminant(kind)
    }

    /// Consumes the next token if it matches a token kind and returns whether
    /// it was consumed.
    fn eat(&mut self, kind: &Token) -> bool {
        let is_match = self.check(kind);

        if is_match {
            self.bump();
        }

        is_match
    }

    /// Consumes the next token with an expected token kind.
    fn expect(&mut self, expected: Token) -> Result<(), SyntaxError> {
        if self.eat(&expected) {
            Ok(())
        } else {
            let actual = self.next();
            Err(SyntaxError::UnexpectedToken { expected, actual })
        }
    }
}
