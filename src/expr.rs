use crate::token::Token;

/// A mathematical expression.
pub enum Expr {
    /// A number expression.
    Number(f64),

    /// A parenthesized expression.
    Paren(Box<Expr>),

    /// A negation expression.
    Negate(Box<Expr>),

    /// A binary expression.
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}

impl Expr {
    /// Evaluates the expression.
    pub fn evaluate(&self) -> f64 {
        match self {
            Self::Number(value) => *value,
            Self::Paren(expr) => expr.evaluate(),
            Self::Negate(expr) => -expr.evaluate(),
            Self::Binary { lhs, op, rhs } => {
                let lhs = lhs.evaluate();
                let rhs = rhs.evaluate();
                op.execute(lhs, rhs)
            }
        }
    }
}

/// A binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// An addition operator.
    Add,

    /// A subtraction operator.
    Subtract,

    /// A multiplication operator.
    Multiply,

    /// A division operator.
    Divide,
}

impl BinOp {
    /// Executes the binary operator.
    fn execute(self, lhs: f64, rhs: f64) -> f64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Subtract => lhs - rhs,
            Self::Multiply => lhs * rhs,
            Self::Divide => lhs / rhs,
        }
    }
}

impl TryFrom<Token> for BinOp {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Add => Ok(Self::Add),
            Token::Subtract => Ok(Self::Subtract),
            Token::Multiply => Ok(Self::Multiply),
            Token::Divide => Ok(Self::Divide),
            _ => Err(()),
        }
    }
}
