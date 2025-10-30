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
        ast::Expr::Unary(op, expr) => resolve_expr_unary(*op, expr),
        ast::Expr::Binary(op, lhs, rhs) => resolve_expr_binary(*op, lhs, rhs),
    }
}

/// Resolves a unary [`ast::Expr`] to an [`hir::Expr`].
fn resolve_expr_unary(op: ast::UnOp, expr: &ast::Expr) -> hir::Expr {
    match op {
        ast::UnOp::Negate => {
            let op = hir::BinOp::Subtract;
            let lhs = hir::Expr::Number(0.0);
            let rhs = resolve_expr(expr);
            hir::Expr::Binary(op, lhs.into(), rhs.into())
        }
    }
}

/// Resolves a binary [`ast::Expr`] to a unary [`hir::Expr`]
fn resolve_expr_binary(op: ast::BinOp, lhs: &ast::Expr, rhs: &ast::Expr) -> hir::Expr {
    let lhs = resolve_expr(lhs);
    let rhs = resolve_expr(rhs);

    let op = match op {
        ast::BinOp::Add => hir::BinOp::Add,
        ast::BinOp::Subtract => hir::BinOp::Subtract,
        ast::BinOp::Multiply => hir::BinOp::Multiply,
        ast::BinOp::Divide => hir::BinOp::Divide,
    };

    hir::Expr::Binary(op, lhs.into(), rhs.into())
}
