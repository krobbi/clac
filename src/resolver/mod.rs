mod env;
mod resolve_error;

use crate::{
    ast::{self, Ast},
    hir::{self, Hir},
};

use self::{
    env::{Env, Location},
    resolve_error::ResolveError,
};

/// Resolves an [`Ast`] to [`Hir`]. This function returns a [`ResolveError`] if
/// the [`Ast`] could not be resolved.
pub fn resolve_ast(ast: &Ast) -> Result<Hir, ResolveError> {
    let mut resolver = Resolver::new();
    resolver.resolve_ast(ast)
}

/// A structure that resolves an [`Ast`] to [`Hir`].
struct Resolver {
    /// The environment of defined variables.
    env: Env,
}

impl Resolver {
    /// Creates a new `Resolver`.
    fn new() -> Self {
        let env = Env::new();
        Self { env }
    }

    /// Resolves an [`Ast`] to [`Hir`]. This function returns a [`ResolveError`]
    /// if the [`Ast`] could not be resolved.
    fn resolve_ast(&mut self, ast: &Ast) -> Result<Hir, ResolveError> {
        let mut stmts = Vec::with_capacity(ast.0.len());

        for stmt in &ast.0 {
            let stmt = self.resolve_stmt(stmt)?;
            stmts.push(stmt);
        }

        Ok(Hir(stmts))
    }

    /// Resolves an [`ast::Stmt`] to an [`hir::Stmt`]. This function returns a
    /// [`ResolveError`] if the [`ast::Stmt`] could not be resolved.
    fn resolve_stmt(&mut self, stmt: &ast::Stmt) -> Result<hir::Stmt, ResolveError> {
        match stmt {
            ast::Stmt::Assign(target, source) => self.resolve_stmt_assign(target, source),
            ast::Stmt::Expr(expr) => {
                let expr = self.resolve_expr(expr)?;

                let stmt = if self.env.is_global() {
                    hir::Stmt::Print(expr.into())
                } else {
                    hir::Stmt::Expr(expr.into())
                };

                Ok(stmt)
            }
        }
    }

    /// Resolves an assignment [`ast::Stmt`] to an [`hir::Stmt`]. This function
    /// returns a [`ResolveError`] if a variable could not be assigned.
    fn resolve_stmt_assign(
        &mut self,
        target: &ast::Expr,
        source: &ast::Expr,
    ) -> Result<hir::Stmt, ResolveError> {
        let ast::Expr::Ident(name) = target else {
            return Err(ResolveError::InvalidAssignTarget);
        };

        let value = self.resolve_expr(source)?;

        match self.env.define(name) {
            None => Err(ResolveError::AlreadyDefinedVariable(name.to_owned())),
            Some(Location::Global) => Ok(hir::Stmt::AssignGlobal(name.to_owned(), value.into())),
            Some(Location::Local(_)) => Ok(hir::Stmt::DefineLocal(value.into())),
        }
    }

    /// Resolves an [`ast::Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if the [`ast::Expr`] could not be resolved.
    fn resolve_expr(&mut self, expr: &ast::Expr) -> Result<hir::Expr, ResolveError> {
        match expr {
            ast::Expr::Number(value) => Ok(hir::Expr::Number(*value)),
            ast::Expr::Ident(name) => self.resolve_expr_ident(name),
            ast::Expr::Paren(expr) => self.resolve_expr(expr),
            ast::Expr::Block(stmts) => self.resolve_expr_block(stmts),
            ast::Expr::Call(..) => todo!("lowering of `ast::Expr::Call`"),
            ast::Expr::Unary(op, expr) => self.resolve_expr_unary(*op, expr),
            ast::Expr::Binary(op, lhs, rhs) => self.resolve_expr_binary(*op, lhs, rhs),
        }
    }

    /// Resolves an identifier [`ast::Expr`] to an [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if the identifier is undefined in the
    /// current environment.
    fn resolve_expr_ident(&self, name: &str) -> Result<hir::Expr, ResolveError> {
        match self.env.find(name) {
            None => Err(ResolveError::UndefinedVariable(name.to_owned())),
            Some(Location::Global) => Ok(hir::Expr::Global(name.to_owned())),
            Some(Location::Local(index)) => Ok(hir::Expr::Local(index)),
        }
    }

    /// Resolves a block [`ast::Expr`] to an [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if any of the block's [`ast::Stmt`]s could
    /// not be resolved.
    fn resolve_expr_block(&mut self, stmts: &[ast::Stmt]) -> Result<hir::Expr, ResolveError> {
        self.env.push_scope();
        let mut block_stmts = Vec::with_capacity(stmts.len());

        for stmt in stmts {
            let stmt = match self.resolve_stmt(stmt) {
                Ok(stmt) => stmt,
                Err(error) => {
                    self.env.pop_scope();
                    return Err(error);
                }
            };

            block_stmts.push(stmt);
        }

        let local_count = self.env.local_count();
        self.env.pop_scope();
        Ok(hir::Expr::Block(local_count, block_stmts))
    }

    /// Resolves a unary [`ast::Expr`] to an [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if the operand could not be resolved.
    fn resolve_expr_unary(
        &mut self,
        op: ast::UnOp,
        expr: &ast::Expr,
    ) -> Result<hir::Expr, ResolveError> {
        let expr = self.resolve_expr(expr)?;

        match op {
            ast::UnOp::Negate => {
                let op = hir::BinOp::Subtract;
                let lhs = hir::Expr::Number(0.0);
                Ok(hir::Expr::Binary(op, lhs.into(), expr.into()))
            }
        }
    }

    /// Resolves a binary [`ast::Expr`] to a unary [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if either operand could not be resolved.
    fn resolve_expr_binary(
        &mut self,
        op: ast::BinOp,
        lhs: &ast::Expr,
        rhs: &ast::Expr,
    ) -> Result<hir::Expr, ResolveError> {
        let lhs = self.resolve_expr(lhs)?;
        let rhs = self.resolve_expr(rhs)?;

        let op = match op {
            ast::BinOp::Add => hir::BinOp::Add,
            ast::BinOp::Subtract => hir::BinOp::Subtract,
            ast::BinOp::Multiply => hir::BinOp::Multiply,
            ast::BinOp::Divide => hir::BinOp::Divide,
        };

        Ok(hir::Expr::Binary(op, lhs.into(), rhs.into()))
    }
}
