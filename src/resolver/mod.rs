mod resolve_error;
mod variables;
mod voidable;

pub use self::resolve_error::ResolveError;

use std::result;

use crate::{
    ast::{Ast, BinOp, Expr, Stmt, UnOp},
    hir::{self, Hir},
};

use self::{
    variables::{ScopeKind, ScopeStack},
    voidable::Voidable,
};

/// A [`Result`][result::Result] that may contain a [`ResolveError`].
type Result<T> = result::Result<T, ResolveError>;

/// Resolves an [`Ast`] to [`Hir`] with an [`Iterator`] over defined global
/// variable names. This function returns a [`ResolveError`] if the [`Ast`]
/// could not be resolved.
pub fn resolve_ast<'a>(ast: &Ast, globals: impl Iterator<Item = &'a String>) -> Result<Hir> {
    let mut resolver = Resolver::new();

    for global in globals {
        resolver.scope_stack.define_variable(global);
    }

    resolver.resolve_ast(ast)
}

/// A structure that resolves an [`Ast`] to [`Hir`].
struct Resolver {
    /// The [`ScopeStack`] for tracking variable definitions.
    scope_stack: ScopeStack,
}

impl Resolver {
    /// Creates a new `Resolver`.
    fn new() -> Self {
        let scope_stack = ScopeStack::new();
        Self { scope_stack }
    }

    /// Resolves an [`Ast`] to [`Hir`]. This function returns a [`ResolveError`]
    /// if the [`Ast`] could not be resolved.
    fn resolve_ast(&mut self, ast: &Ast) -> Result<Hir> {
        let stmts = self.resolve_stmts(&ast.0, ScopeKind::Global)?;
        Ok(Hir(stmts))
    }

    /// Resolves a slice of [`Stmt`]s to a [`Vec`] of [`hir::Stmt`]s. This
    /// function returns a [`ResolveError`] if the [`Stmt`]s could not be
    /// resolved.
    fn resolve_stmts(&mut self, stmts: &[Stmt], scope_kind: ScopeKind) -> Result<Vec<hir::Stmt>> {
        let mut resolved_stmts = Vec::with_capacity(stmts.len());

        for stmt in stmts {
            let stmt = match stmt {
                Stmt::Assign(target, source) => {
                    self.resolve_stmt_assign(target, source, scope_kind)?
                }
                Stmt::Expr(expr) => match (self.resolve_expr_voidable(expr)?, scope_kind) {
                    (Voidable::Nop, _) => continue,
                    (Voidable::Expr(expr), ScopeKind::Local) => hir::Stmt::Expr(expr.into()),
                    (Voidable::Expr(value), ScopeKind::Global) => hir::Stmt::Print(value.into()),
                    (Voidable::Stmt(stmt), _) => stmt,
                },
            };

            resolved_stmts.push(stmt);
        }

        Ok(resolved_stmts)
    }

    /// Resolves an assignment [`Stmt`] to an [`hir::Stmt`]. This function
    /// returns a [`ResolveError`] if the source [`Expr`] is void or could not
    /// be resolved, or if the target [`Expr`] is invalid.
    fn resolve_stmt_assign(
        &mut self,
        target: &Expr,
        source: &Expr,
        scope_kind: ScopeKind,
    ) -> Result<hir::Stmt> {
        let value = self.resolve_expr(source)?;

        let name = match target {
            Expr::Ident(name) => name,
            Expr::Call(..) => todo!("resolving function definitions"),
            _ => return Err(ResolveError::InvalidAssignTarget),
        };

        if self.scope_stack.has_inner_variable(name) {
            return Err(ResolveError::AlreadyDefinedVariable(name.to_owned()));
        }

        let stmt = match scope_kind {
            ScopeKind::Local => hir::Stmt::DefineLocal(name.to_owned(), value.into()),
            ScopeKind::Global => hir::Stmt::AssignGlobal(name.to_owned(), value.into()),
        };

        self.scope_stack.define_variable(name);
        Ok(stmt)
    }

    /// Resolves an [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if the [`Expr`] is void or could not be resolved.
    fn resolve_expr(&mut self, expr: &Expr) -> Result<hir::Expr> {
        match self.resolve_expr_voidable(expr)? {
            Voidable::Expr(expr) => Ok(expr),
            _ => Err(ResolveError::VoidArgument),
        }
    }

    /// Resolves an [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the [`Expr`] could not be resolved.
    fn resolve_expr_voidable(&mut self, expr: &Expr) -> Result<Voidable> {
        let expr = match expr {
            Expr::Number(value) => hir::Expr::Number(*value),
            Expr::Ident(name) => self.resolve_expr_ident(name)?,
            Expr::Paren(expr) => self.resolve_expr(expr)?,
            Expr::Block(stmts) => return self.resolve_expr_block(stmts),
            Expr::Call(..) => todo!("resolving `Expr::Call`"),
            Expr::Unary(op, rhs) => self.resolve_expr_unary(*op, rhs)?,
            Expr::Binary(op, lhs, rhs) => self.resolve_expr_binary(*op, lhs, rhs)?,
        };

        Ok(expr.into())
    }

    /// Resolves an identifier [`Expr`] to an [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if the identifier is not a defined variable.
    fn resolve_expr_ident(&self, name: &str) -> Result<hir::Expr> {
        match self.scope_stack.resolve_variable(name) {
            None => Err(ResolveError::UndefinedVariable(name.to_owned())),
            Some(ScopeKind::Local) => Ok(hir::Expr::Local(name.to_owned())),
            Some(ScopeKind::Global) => Ok(hir::Expr::Global(name.to_owned())),
        }
    }

    /// Resolves a block [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the block's [`Stmt`]s could not be resolved.
    fn resolve_expr_block(&mut self, stmts: &[Stmt]) -> Result<Voidable> {
        self.scope_stack.push_scope();
        let mut stmts = self.resolve_stmts(stmts, ScopeKind::Local)?;
        self.scope_stack.pop_scope();

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
    fn resolve_expr_unary(&mut self, op: UnOp, rhs: &Expr) -> Result<hir::Expr> {
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
    fn resolve_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<hir::Expr> {
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
