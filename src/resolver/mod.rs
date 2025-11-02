mod resolve_error;
mod scope_kind;
mod voidable;

pub use self::resolve_error::ResolveError;

use crate::{
    ast::{Ast, BinOp, Expr, Stmt, UnOp},
    hir::{self, Hir},
};

use self::{scope_kind::ScopeKind, voidable::Voidable};

/// Resolves an [`Ast`] to [`Hir`]. This function returns a [`ResolveError`] if
/// the [`Ast`] could not be resolved.
pub fn resolve_ast(ast: &Ast) -> Result<Hir, ResolveError> {
    let resolver = Resolver::new();
    resolver.resolve_ast(ast)
}

/// A structure that resolves an [`Ast`] to [`Hir`].
struct Resolver;

impl Resolver {
    /// Creates a new `Resolver`.
    fn new() -> Self {
        Self
    }

    /// Resolves an [`Ast`] to [`Hir`]. This function returns a [`ResolveError`]
    /// if the [`Ast`] could not be resolved.
    fn resolve_ast(&self, ast: &Ast) -> Result<Hir, ResolveError> {
        let stmts = self.resolve_stmts(&ast.0, ScopeKind::Global)?;
        Ok(Hir(stmts))
    }

    /// Resolves a slice of [`Stmt`]s to a [`Vec`] of [`hir::Stmt`]s. This
    /// function returns a [`ResolveError`] if the [`Stmt`]s could not be
    /// resolved.
    fn resolve_stmts(
        &self,
        stmts: &[Stmt],
        scope_kind: ScopeKind,
    ) -> Result<Vec<hir::Stmt>, ResolveError> {
        let mut resolved_stmts = Vec::with_capacity(stmts.len());

        for stmt in stmts {
            let stmt = match stmt {
                Stmt::Assign(..) => todo!("resolving `Stmt::Assign`"),
                Stmt::Expr(expr) => match (self.resolve_expr_voidable(expr)?, scope_kind) {
                    (Voidable::Nop, _) => continue,
                    (Voidable::Expr(expr), ScopeKind::Global) => hir::Stmt::Print(expr.into()),
                    (Voidable::Expr(expr), ScopeKind::Local) => hir::Stmt::Expr(expr.into()),
                    (Voidable::Stmt(stmt), _) => stmt,
                },
            };

            resolved_stmts.push(stmt);
        }

        Ok(resolved_stmts)
    }

    /// Resolves an [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if the [`Expr`] is void or could not be resolved.
    fn resolve_expr(&self, expr: &Expr) -> Result<hir::Expr, ResolveError> {
        match self.resolve_expr_voidable(expr)? {
            Voidable::Expr(expr) => Ok(expr),
            _ => Err(ResolveError::VoidArgument),
        }
    }

    /// Resolves an [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the [`Expr`] could not be resolved.
    fn resolve_expr_voidable(&self, expr: &Expr) -> Result<Voidable, ResolveError> {
        let expr = match expr {
            Expr::Number(value) => hir::Expr::Number(*value),
            Expr::Ident(..) => todo!("resolving `Expr::Ident`"),
            Expr::Paren(expr) => self.resolve_expr(expr)?,
            Expr::Block(stmts) => return self.resolve_expr_block(stmts),
            Expr::Call(..) => todo!("resolving `Expr::Call`"),
            Expr::Unary(op, rhs) => self.resolve_expr_unary(*op, rhs)?,
            Expr::Binary(op, lhs, rhs) => self.resolve_expr_binary(*op, lhs, rhs)?,
        };

        Ok(expr.into())
    }

    /// Resolves a block [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the block's [`Stmt`]s could not be resolved.
    fn resolve_expr_block(&self, stmts: &[Stmt]) -> Result<Voidable, ResolveError> {
        let mut stmts = self.resolve_stmts(stmts, ScopeKind::Local)?;

        let block = match stmts.pop() {
            None => Voidable::Nop,
            Some(hir::Stmt::Expr(expr)) => {
                let expr = if stmts.is_empty() {
                    *expr
                } else {
                    hir::Expr::Block(stmts, expr)
                };

                expr.into()
            }
            Some(stmt) => {
                stmts.push(stmt);
                hir::Stmt::Block(stmts).into()
            }
        };

        Ok(block)
    }

    /// Resolves a unary [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if the operand is void or could not be resolved.
    fn resolve_expr_unary(&self, op: UnOp, rhs: &Expr) -> Result<hir::Expr, ResolveError> {
        let rhs = self.resolve_expr(rhs)?;

        match op {
            UnOp::Negate => {
                let op = hir::BinOp::Subtract;
                let lhs = hir::Expr::Number(0.0);
                Ok(hir::Expr::Binary(op, lhs.into(), rhs.into()))
            }
        }
    }

    /// Resolves a binary [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if either operand is void or could not be resolved.
    fn resolve_expr_binary(
        &self,
        op: BinOp,
        lhs: &Expr,
        rhs: &Expr,
    ) -> Result<hir::Expr, ResolveError> {
        let lhs = self.resolve_expr(lhs)?;
        let rhs = self.resolve_expr(rhs)?;

        let op = match op {
            BinOp::Add => hir::BinOp::Add,
            BinOp::Subtract => hir::BinOp::Subtract,
            BinOp::Multiply => hir::BinOp::Multiply,
            BinOp::Divide => hir::BinOp::Divide,
        };

        Ok(hir::Expr::Binary(op, lhs.into(), rhs.into()))
    }
}
