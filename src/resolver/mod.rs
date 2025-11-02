#![expect(
    clippy::only_used_in_recursion,
    reason = "resolver fields should be added later"
)]

use crate::{
    ast::{Ast, Expr, Stmt},
    hir::{self, Hir},
};

/// Resolves an [`Ast`] to [`Hir`].
pub fn resolve_ast(ast: &Ast) -> Hir {
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

    /// Resolves an [`Ast`] to [`Hir`].
    fn resolve_ast(&self, ast: &Ast) -> Hir {
        let mut stmts = Vec::with_capacity(ast.0.len());

        for stmt in &ast.0 {
            let stmt = match stmt {
                Stmt::Assign(..) => todo!("resolving `Stmt::Assign`"),
                Stmt::Expr(expr) => {
                    let expr = self.resolve_expr(expr);
                    hir::Stmt::Print(expr)
                }
            };

            stmts.push(stmt);
        }

        Hir(stmts)
    }

    /// Resolves an [`Expr`] to an [`hir::Expr`]
    fn resolve_expr(&self, expr: &Expr) -> hir::Expr {
        match expr {
            Expr::Number(value) => hir::Expr::Number(*value),
            Expr::Ident(..) => todo!("resolving `Expr::Ident`"),
            Expr::Paren(expr) => self.resolve_expr(expr),
            Expr::Block(..) => todo!("resolving `Expr::Block`"),
            Expr::Call(..) => todo!("resolving `Expr::Call`"),
            Expr::Unary(..) => todo!("resolving `Expr::Unary`"),
            Expr::Binary(..) => todo!("resolving `Expr::Binary`"),
        }
    }
}
