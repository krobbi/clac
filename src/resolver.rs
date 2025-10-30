use crate::{
    ast::{self, Ast},
    hir::{self, Hir},
};

/// Resolves an [`Ast`] to [`Hir`].
pub fn resolve_ast(ast: &Ast) -> Hir {
    let mut stmts = Vec::with_capacity(ast.0.len());

    for stmt in &ast.0 {
        let stmt = resolve_stmt(stmt);
        stmts.push(stmt);
    }

    Hir(stmts)
}

/// Resolves an [`ast::Stmt`] to an [`hir::Stmt`].
fn resolve_stmt(stmt: &ast::Stmt) -> hir::Stmt {
    match stmt {
        ast::Stmt::Assign(..) => todo!("lowering of `ast::Stmt::Assign`"),
        ast::Stmt::Expr(expr) => {
            let expr = resolve_expr(expr);
            hir::Stmt::Print(expr.into())
        }
    }
}

/// Resolves an [`ast::Expr`] to an [`hir::Expr`].
fn resolve_expr(expr: &ast::Expr) -> hir::Expr {
    match expr {
        ast::Expr::Number(value) => hir::Expr::Number(*value),
        ast::Expr::Ident(..) => todo!("lowering of `ast::Expr::Ident`"),
        ast::Expr::Paren(expr) => resolve_expr(expr),
        ast::Expr::Block(..) => todo!("lowering of `ast::Expr::Block`"),
        ast::Expr::Call(..) => todo!("lowering of `ast::Expr::Call`"),
        ast::Expr::Unary(..) => todo!("lowering of `ast::Expr::Unary`"),
        ast::Expr::Binary(..) => todo!("lowering of `ast::Expr::Binary`"),
    }
}
