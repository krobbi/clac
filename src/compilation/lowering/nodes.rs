use super::super::ir::{Expr, Stmt};

/// An [`Ir`][super::super::ir::Ir] node which is either an [`Expr`] or a
/// [`Stmt`].
pub enum Node {
    /// A [`Stmt`].
    Stmt(Stmt),

    /// An [`Expr`].
    Expr(Expr),
}

impl From<Stmt> for Node {
    fn from(value: Stmt) -> Self {
        Self::Stmt(value)
    }
}

impl From<Expr> for Node {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}
