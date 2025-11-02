use crate::{
    ast::{Ast, BinOp, Expr, Stmt, UnOp},
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
            Expr::Unary(op, rhs) => self.resolve_expr_unary(*op, rhs),
            Expr::Binary(op, lhs, rhs) => self.resolve_expr_binary(*op, lhs, rhs),
        }
    }

    /// Resolves a unary [`Expr`] to an [`hir::Expr`].
    fn resolve_expr_unary(&self, op: UnOp, rhs: &Expr) -> hir::Expr {
        let rhs = self.resolve_expr(rhs);

        match op {
            UnOp::Negate => {
                let op = hir::BinOp::Subtract;
                let lhs = hir::Expr::Number(0.0);
                hir::Expr::Binary(op, lhs.into(), rhs.into())
            }
        }
    }

    /// Resolves a binary [`Expr`] to an [`hir::Expr`].
    fn resolve_expr_binary(&self, op: BinOp, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let lhs = self.resolve_expr(lhs);
        let rhs = self.resolve_expr(rhs);

        let op = match op {
            BinOp::Add => hir::BinOp::Add,
            BinOp::Subtract => hir::BinOp::Subtract,
            BinOp::Multiply => hir::BinOp::Multiply,
            BinOp::Divide => hir::BinOp::Divide,
        };

        hir::Expr::Binary(op, lhs.into(), rhs.into())
    }
}
