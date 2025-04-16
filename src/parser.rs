use std::{error, fmt, iter, mem};

use crate::{
    lexer::{LexError, Lexer},
    token::Token,
};

/// Parses a program from source code.
pub fn parse_source(source: &str) -> Result<Vec<String>, ParseError> {
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
    fn parse_program(&mut self) -> Result<Vec<String>, ParseError> {
        let mut program = vec![];

        while !self.check(&Token::Eof)? {
            program.push(self.parse_expr()?);
        }

        Ok(program)
    }

    /// Parses a top-level expression.
    fn parse_expr(&mut self) -> Result<String, ParseError> {
        self.parse_atom()
    }

    /// Parses an atom expression.
    fn parse_atom(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Token::Literal(value) => Ok(value.to_string()),
            Token::OpenParen => {
                let expr = self.parse_expr()?;
                self.expect(Token::CloseParen)?;
                Ok(format!("(group {expr})"))
            }
            Token::Minus => Ok(format!("(negate {})", self.parse_atom()?)),
            t => Err(ParseError::NonExpression(t)),
        }
    }

    /// Returns whether the next token matches a token kind.
    fn check(&mut self, kind: &Token) -> Result<bool, LexError> {
        match self.lexer.peek().unwrap() {
            Ok(t) => Ok(mem::discriminant(t) == mem::discriminant(kind)),
            Err(e) => Err(e.clone()),
        }
    }

    /// Consumes the next token with an expected token kind.
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let actual = self.next()?;

        if mem::discriminant(&actual) == mem::discriminant(&expected) {
            Ok(())
        } else {
            Err(ParseError::Unexpected { expected, actual })
        }
    }

    /// Consumes and returns the next token.
    fn next(&mut self) -> Result<Token, LexError> {
        self.lexer.next().unwrap()
    }
}
