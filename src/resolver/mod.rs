mod resolve_error;
mod scope_stack;
mod voidable;

pub use self::resolve_error::ResolveError;

use std::result;

use crate::{
    ast::{Ast, BinOp, Expr, Stmt, UnOp},
    hir::{self, Hir},
};

use self::{
    resolve_error::ExprArea,
    scope_stack::{ScopeKind, ScopeStack},
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
                Stmt::Expr(expr) => {
                    let voidable = self.resolve_expr_voidable(expr)?;

                    match (voidable, scope_kind) {
                        (Voidable::Expr(expr), ScopeKind::Local) => hir::Stmt::Expr(expr.into()),
                        (Voidable::Expr(expr), ScopeKind::Global) => hir::Stmt::Print(expr.into()),
                        (Voidable::Stmt(stmt), _) => stmt,
                    }
                }
            };

            resolved_stmts.push(stmt);
        }

        Ok(resolved_stmts)
    }

    /// Resolves an assignment [`Stmt`] to an [`hir::Stmt`]. This function
    /// returns a [`ResolveError`] if the source [`Expr`] is a statement or
    /// could not be resolved, or if the target [`Expr`] is invalid.
    fn resolve_stmt_assign(
        &mut self,
        target: &Expr,
        source: &Expr,
        scope_kind: ScopeKind,
    ) -> Result<hir::Stmt> {
        let (name, value) = match target {
            Expr::Ident(name) => (name, self.resolve_expr(source, ExprArea::AssignSource)?),
            Expr::Call(callee, args) => {
                let Expr::Ident(name) = callee.as_ref() else {
                    return Err(ResolveError::InvalidFunctionName);
                };

                (name, self.resolve_expr_function(args, source)?)
            }
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

    /// Resolves an [`Expr`] to an [`hir::Expr`] in an [`ExprArea`]. This
    /// function returns a [`ResolveError`] if the [`Expr`] is a statement or
    /// could not be resolved.
    fn resolve_expr(&mut self, expr: &Expr, area: ExprArea) -> Result<hir::Expr> {
        match self.resolve_expr_voidable(expr)? {
            Voidable::Expr(expr) => Ok(expr),
            Voidable::Stmt(_) => Err(ResolveError::UsedStmt(area)),
        }
    }

    /// Resolves an [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the [`Expr`] could not be resolved.
    fn resolve_expr_voidable(&mut self, expr: &Expr) -> Result<Voidable> {
        let expr = match expr {
            Expr::Number(value) => hir::Expr::Number(*value),
            Expr::Ident(name) => self.resolve_expr_ident(name)?,
            Expr::Paren(expr) => self.resolve_expr(expr, ExprArea::Paren)?,
            Expr::Block(stmts) => return self.resolve_expr_block(stmts),
            Expr::Call(callee, args) => self.resolve_expr_call(callee, args)?,
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

    /// Resolves a function [`Expr`] to an [`hir::Expr`]. This function returns
    /// a [`ResolveError`] if the function has an invalid signature or if the
    /// function body is a statement.
    fn resolve_expr_function(&mut self, params: &[Expr], body: &Expr) -> Result<hir::Expr> {
        let mut resolved_params = Vec::with_capacity(params.len());

        for param in params {
            let Expr::Ident(param) = param else {
                return Err(ResolveError::InvalidParam);
            };

            if resolved_params.contains(param) {
                return Err(ResolveError::DuplicateParam(param.to_owned()));
            }

            resolved_params.push(param.to_owned());
        }

        self.scope_stack.begin_function(&resolved_params);
        let body = self.resolve_expr(body, ExprArea::FunctionBody)?;
        self.scope_stack.end_function();
        Ok(hir::Expr::Function(resolved_params, body.into()))
    }

    /// Resolves a block [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the block's [`Stmt`]s could not be resolved.
    fn resolve_expr_block(&mut self, stmts: &[Stmt]) -> Result<Voidable> {
        self.scope_stack.push_scope();
        let mut stmts = self.resolve_stmts(stmts, ScopeKind::Local)?;
        self.scope_stack.pop_scope();

        let block = match stmts.pop() {
            None => hir::Stmt::Nop.into(),
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

    /// Resolves a function call [`Expr`] to an [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if the callee or arguments are statements or
    /// could not be resolved.
    fn resolve_expr_call(&mut self, callee: &Expr, args: &[Expr]) -> Result<hir::Expr> {
        let callee = self.resolve_expr(callee, ExprArea::Callee)?;
        let mut resolved_args = Vec::with_capacity(args.len());

        for arg in args {
            let arg = self.resolve_expr(arg, ExprArea::Arg)?;
            resolved_args.push(arg);
        }

        Ok(hir::Expr::Call(callee.into(), resolved_args))
    }

    /// Resolves a unary [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if the operand is a statement or could not be resolved.
    fn resolve_expr_unary(&mut self, op: UnOp, rhs: &Expr) -> Result<hir::Expr> {
        let rhs = self.resolve_expr(rhs, ExprArea::Operand)?;

        match op {
            UnOp::Negate => {
                let op = hir::BinOp::Subtract;
                let lhs = hir::Expr::Number(0.0);
                Ok(hir::Expr::Binary(op, lhs.into(), rhs.into()))
            }
        }
    }

    /// Resolves a binary [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if either operand is a statement or could not be
    /// resolved.
    fn resolve_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<hir::Expr> {
        let lhs = self.resolve_expr(lhs, ExprArea::Operand)?;
        let rhs = self.resolve_expr(rhs, ExprArea::Operand)?;

        let op = match op {
            BinOp::Add => hir::BinOp::Add,
            BinOp::Subtract => hir::BinOp::Subtract,
            BinOp::Multiply => hir::BinOp::Multiply,
            BinOp::Divide => hir::BinOp::Divide,
        };

        Ok(hir::Expr::Binary(op, lhs.into(), rhs.into()))
    }
}
