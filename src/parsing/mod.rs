#[cfg(test)]
mod tests;

mod errors;
mod lexer;
mod tokens;

use std::mem;

use thiserror::Error;

use crate::ast::{Ast, BinOp, Expr, Literal, LogicOp, UnOp};

use self::{
    errors::ErrorKind,
    lexer::Lexer,
    tokens::{Token, TokenType},
};

/// An error caught while parsing an [`Ast`] from source code.
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct ParsingError(Box<ErrorKind>);

/// Parses an [`Ast`] from source code. This function returns a [`ParsingError`]
/// if an [`Ast`] could not be parsed.
pub fn parse_source(source: &str) -> Result<Ast, ParsingError> {
    let mut parser = Parser::new(source);
    let ast = parser.parse_ast();
    parser.error.map_or(Ok(ast), Err)
}

/// A structure which parses an [`Ast`] from source code.
struct Parser<'src> {
    /// The [`Lexer`] for reading [`Token`]s from source code.
    lexer: Lexer<'src>,

    /// The next [`Token`].
    next_token: Token,

    /// The first [`ParsingError`], if any.
    error: Option<ParsingError>,
}

impl<'src> Parser<'src> {
    /// Creates a new `Parser` from source code.
    fn new(source: &'src str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(source),
            next_token: Token::Eof,
            error: None,
        };

        parser.bump();
        parser
    }

    /// Parses an [`Ast`].
    fn parse_ast(&mut self) -> Ast {
        Ast(self.parse_sequence(TokenType::Eof))
    }

    /// Parses a sequence of statement [`Expr`]s until the next [`Token`]
    /// matches a terminator [`TokenType`].
    fn parse_sequence(&mut self, terminator: TokenType) -> Box<[Expr]> {
        let mut stmts = Vec::new();

        while !self.is_terminated(terminator) {
            stmts.push(self.parse_stmt());
            self.eat(TokenType::Comma);
        }

        stmts.into()
    }

    /// Parses a statement [`Expr`].
    fn parse_stmt(&mut self) -> Expr {
        self.parse_expr()
    }

    /// Parses an [`Expr`].
    fn parse_expr(&mut self) -> Expr {
        self.parse_expr_assignment()
    }

    /// Parses an assignment [`Expr`].
    fn parse_expr_assignment(&mut self) -> Expr {
        let lhs = self.parse_expr_mapping();

        if self.eat(TokenType::Equals) {
            let source = self.parse_expr_mapping();

            if self.peek() == TokenType::Equals {
                self.report_error(ErrorKind::ChainedAssignment);
            }

            Expr::Assign(lhs.into(), source.into())
        } else {
            lhs
        }
    }

    /// Parses a function [`Expr`] or a ternary conditional [`Expr`].
    fn parse_expr_mapping(&mut self) -> Expr {
        let lhs = self.parse_expr_or();

        match self.peek() {
            TokenType::MinusGreater => {
                self.bump(); // Consume the operator token.
                let body = self.parse_expr_mapping();
                Expr::Function(unwrap_list(lhs), body.into())
            }
            TokenType::Question => {
                self.bump(); // Consume the operator token.
                let then_expr = self.parse_expr();
                self.expect(TokenType::Colon);
                let else_expr = self.parse_expr_mapping();
                Expr::Cond(lhs.into(), then_expr.into(), else_expr.into())
            }
            _ => lhs,
        }
    }

    /// Parses a logical or [`Expr`].
    fn parse_expr_or(&mut self) -> Expr {
        let mut lhs = self.parse_expr_and();

        while self.eat(TokenType::PipePipe) {
            let rhs = self.parse_expr_and();
            lhs = Expr::Logic(LogicOp::Or, lhs.into(), rhs.into());
        }

        lhs
    }

    /// Parses a logical and [`Expr`].
    fn parse_expr_and(&mut self) -> Expr {
        let mut lhs = self.parse_expr_comparison();

        while self.eat(TokenType::AndAnd) {
            let rhs = self.parse_expr_comparison();
            lhs = Expr::Logic(LogicOp::And, lhs.into(), rhs.into());
        }

        lhs
    }

    /// Parses a comparison [`Expr`].
    pub fn parse_expr_comparison(&mut self) -> Expr {
        let lhs = self.parse_expr_sum();

        match BinOp::comparison_from_token_type(self.peek()) {
            None => lhs,
            Some(op) => {
                self.bump(); // Consume the operator token.
                let rhs = self.parse_expr_sum();

                if BinOp::comparison_from_token_type(self.peek()).is_some() {
                    self.report_error(ErrorKind::ChainedComparison);
                }

                Expr::Binary(op, lhs.into(), rhs.into())
            }
        }
    }

    /// Parses a sum [`Expr`].
    pub fn parse_expr_sum(&mut self) -> Expr {
        let mut lhs = self.parse_expr_term();

        while let Some(op) = BinOp::sum_from_token_type(self.peek()) {
            self.bump(); // Consume the operator token.
            let rhs = self.parse_expr_term();
            lhs = Expr::Binary(op, lhs.into(), rhs.into());
        }

        lhs
    }

    /// Parses a term [`Expr`].
    pub fn parse_expr_term(&mut self) -> Expr {
        let mut lhs = self.parse_expr_prefix();

        while let Some(op) = BinOp::term_from_token_type(self.peek()) {
            self.bump(); // Consume the operator token.
            let rhs = self.parse_expr_prefix();
            lhs = Expr::Binary(op, lhs.into(), rhs.into());
        }

        lhs
    }

    /// Parses a prefix [`Expr`].
    fn parse_expr_prefix(&mut self) -> Expr {
        let mut lhs = match self.bump() {
            Token::Literal(literal) => Expr::Literal(literal),
            Token::Ident(symbol) => Expr::Ident(symbol),
            Token::OpenParen => self.parse_expr_paren(),
            Token::OpenBrace => {
                let stmts = self.parse_sequence(TokenType::CloseBrace);
                self.expect(TokenType::CloseBrace);
                Expr::Block(stmts)
            }
            Token::Minus => {
                let rhs = self.parse_expr_prefix();
                Expr::Unary(UnOp::Negate, rhs.into())
            }
            Token::Bang => {
                let rhs = self.parse_expr_prefix();
                Expr::Unary(UnOp::Not, rhs.into())
            }
            token => {
                self.report_error(ErrorKind::ExpectedExpr(token));
                default_expr()
            }
        };

        while self.eat(TokenType::OpenParen) {
            let args = self.parse_expr_paren();
            lhs = Expr::Call(lhs.into(), unwrap_list(args));
        }

        if self.eat(TokenType::Caret) {
            let rhs = self.parse_expr_prefix();
            lhs = Expr::Binary(BinOp::Power, lhs.into(), rhs.into());
        }

        lhs
    }

    /// Parses a parenthesized [`Expr`] or a tuple [`Expr`] after consuming its
    /// opening parenthesis.
    fn parse_expr_paren(&mut self) -> Expr {
        let mut exprs = Vec::new();

        let is_empty_or_has_trailing_comma = loop {
            if self.is_terminated(TokenType::CloseParen) {
                break true;
            }

            exprs.push(self.parse_expr());

            if !self.eat(TokenType::Comma) {
                break false;
            }
        };

        self.expect(TokenType::CloseParen);

        if is_empty_or_has_trailing_comma || exprs.len() != 1 {
            Expr::Tuple(exprs.into())
        } else {
            Expr::Paren(exprs.pop().expect("parentheses should not be empty").into())
        }
    }

    /// Returns the next [`Token`]'s [`TokenType`].
    const fn peek(&self) -> TokenType {
        self.next_token.token_type()
    }

    /// Returns `true` if the next [`Token`] matches a terminator [`TokenType`]
    /// or is the end of source code.
    fn is_terminated(&self, terminator: TokenType) -> bool {
        let next_token_type = self.peek();
        next_token_type == terminator || next_token_type == TokenType::Eof
    }

    /// Consumes the next [`Token`].
    fn bump(&mut self) -> Token {
        let following_token = loop {
            match self.lexer.bump() {
                Ok(token) => break token,
                Err(error) => self.report_error(ErrorKind::Lexing(error)),
            }
        };

        mem::replace(&mut self.next_token, following_token)
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns [`true`] if a [`Token`] was consumed.
    fn eat(&mut self, expected: TokenType) -> bool {
        let is_match = self.peek() == expected;

        if is_match {
            self.bump();
        }

        is_match
    }

    /// Consumes the next [`Token`] and reports a [`ParsingError`] if it does
    /// not match an expected [`TokenType`].
    fn expect(&mut self, expected: TokenType) {
        let actual = self.bump();

        if actual.token_type() != expected {
            self.report_error(ErrorKind::UnexpectedToken(expected, actual));
        }
    }

    /// Reports an [`ErrorKind`].
    #[cold]
    fn report_error(&mut self, error: ErrorKind) {
        self.error.get_or_insert_with(|| ParsingError(error.into()));
    }
}

impl BinOp {
    /// Returns a comparison `BinOp` from a [`TokenType`]. This function returns
    /// [`None`] if the [`TokenType`] does not correspond to a comparison
    /// `BinOp`.
    const fn comparison_from_token_type(token_type: TokenType) -> Option<Self> {
        let op = match token_type {
            TokenType::EqualsEquals => Self::Equal,
            TokenType::BangEquals => Self::NotEqual,
            TokenType::Less => Self::Less,
            TokenType::LessEquals => Self::LessEqual,
            TokenType::Greater => Self::Greater,
            TokenType::GreaterEquals => Self::GreaterEqual,
            _ => return None,
        };

        Some(op)
    }

    /// Returns a sum `BinOp` from a [`TokenType`]. This function returns
    /// [`None`] if the [`TokenType`] does not correspond to a sum `BinOp`.
    const fn sum_from_token_type(token_type: TokenType) -> Option<Self> {
        let op = match token_type {
            TokenType::Plus => Self::Add,
            TokenType::Minus => Self::Subtract,
            _ => return None,
        };

        Some(op)
    }

    /// Returns a term `BinOp` from a [`TokenType`]. This function returns
    /// [`None`] if the [`TokenType`] does not correspond to a term `BinOp`.
    const fn term_from_token_type(token_type: TokenType) -> Option<Self> {
        let op = match token_type {
            TokenType::Star => Self::Multiply,
            TokenType::Slash => Self::Divide,
            _ => return None,
        };

        Some(op)
    }
}

/// Unwraps a function parameter or call argument list from an [`Expr`].
fn unwrap_list(expr: Expr) -> Box<[Expr]> {
    // Using `expr.into_boxed_slice()` would avoid a reallocation for
    // `Expr::Paren`, but this is unstable.
    // https://github.com/rust-lang/rust/issues/71582
    match expr {
        Expr::Paren(expr) => [*expr].into(),
        Expr::Tuple(exprs) => exprs,
        expr => [expr].into(),
    }
}

/// Returns a default [`Expr`] for error recovery.
const fn default_expr() -> Expr {
    Expr::Literal(Literal::Number(0.0))
}
