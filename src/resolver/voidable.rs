use crate::hir::{Expr, Stmt};

/// An [`Hir`][crate::hir::Hir] node that is either an [`Expr`] or a [`Stmt`].
pub enum Voidable {
    /// An [`Expr`].
    Expr(Expr),

    /// A [`Stmt`].
    Stmt(Stmt),
}

impl From<Expr> for Voidable {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}

impl From<Stmt> for Voidable {
    fn from(value: Stmt) -> Self {
        Self::Stmt(value)
    }
}
