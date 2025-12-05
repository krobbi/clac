mod locals;
mod resolve_error;
mod voidable;

pub use self::resolve_error::ResolveError;

use std::{collections::HashSet, result};

use crate::{
    ast::{Ast, BinOp, Expr, Stmt, UnOp},
    decl_table::DeclTable,
    hir::{self, Hir},
    interpreter::Globals,
};

use self::{locals::Locals, resolve_error::ExprArea, voidable::Voidable};

/// A [`Result`][result::Result] that may contain a [`ResolveError`].
type Result<T> = result::Result<T, ResolveError>;

/// Resolves an [`Ast`] to [`Hir`] with [`Globals`] and a [`DeclTable`]. This
/// function returns a [`ResolveError`] if the [`Ast`] could not be resolved.
pub fn resolve_ast(ast: &Ast, globals: &Globals, decls: &mut DeclTable) -> Result<Hir> {
    let mut resolver = Resolver::new(globals, decls);
    resolver.resolve_ast(ast)
}

/// A structure that resolves an [`Ast`] to [`Hir`].
struct Resolver<'a, 'b> {
    /// The [`Globals`].
    globals: &'a Globals,

    /// The set of newly-declared global variable names.
    new_globals: HashSet<String>,

    /// The [`Locals`].
    locals: Locals<'b>,
}

impl<'a, 'b> Resolver<'a, 'b> {
    /// Creates a new `Resolver` from [`Globals`] and a [`DeclTable`].
    fn new(globals: &'a Globals, decls: &'b mut DeclTable) -> Self {
        Self {
            globals,
            new_globals: HashSet::new(),
            locals: Locals::new(decls),
        }
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

        match scope_kind {
            ScopeKind::Global => self.define_global(name, value),
            ScopeKind::Local => self.define_local(name, value),
        }
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
            Expr::Literal(literal) => Ok(hir::Expr::Literal(literal.clone())),
            Expr::Ident(name) => self.resolve_expr_ident(name),
            Expr::Paren(expr) => self.resolve_expr(expr, ExprArea::Paren),
            Expr::Tuple(_) => Err(ResolveError::TupleValue),
            Expr::Block(stmts) => return self.resolve_expr_block(stmts),
            Expr::Function(params, body) => self.resolve_expr_function(params, body),
            Expr::Call(callee, args) => self.resolve_expr_call(callee, args),
            Expr::Unary(op, rhs) => self.resolve_expr_unary(*op, rhs),
            Expr::Binary(op, lhs, rhs) => self.resolve_expr_binary(*op, lhs, rhs),
            Expr::Cond(_cond, _then, _or) => todo!("resolving conditional expressions"),
        };

        expr.map(Voidable::Expr)
    }

    /// Resolves an identifier [`Expr`] to an [`hir::Expr`]. This function
    /// returns a [`ResolveError`] if the identifier is not a defined variable.
    fn resolve_expr_ident(&mut self, name: &str) -> Result<hir::Expr> {
        #[expect(clippy::option_if_let_else, reason = "better readability")]
        if let Some(id) = self.locals.read(name) {
            Ok(hir::Expr::Local(id))
        } else if self.is_global_defined(name) {
            Ok(hir::Expr::Global(name.to_owned()))
        } else {
            Err(ResolveError::UndefinedVariable(name.to_owned()))
        }
    }

    /// Resolves a block [`Expr`] to a [`Voidable`]. This function returns a
    /// [`ResolveError`] if the block's [`Stmt`]s could not be resolved.
    fn resolve_expr_block(&mut self, stmts: &[Stmt]) -> Result<Voidable> {
        self.locals.begin_block();
        let mut stmts = self.resolve_stmts(stmts, ScopeKind::Local)?;
        self.locals.end_block();

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

    /// Resolves a function [`Expr`] to an [`hir::Expr`]. This function returns
    /// a [`ResolveError`] if the function has an invalid signature or if the
    /// function body is a statement.
    fn resolve_expr_function(&mut self, params: &[Expr], body: &Expr) -> Result<hir::Expr> {
        let mut param_names = Vec::with_capacity(params.len());

        for param in params {
            let Expr::Ident(param) = param else {
                return Err(ResolveError::InvalidParam);
            };

            if param_names.contains(param) {
                return Err(ResolveError::DuplicateParam(param.to_owned()));
            }

            param_names.push(param.to_owned());
        }

        self.locals.begin_function();
        let mut params = Vec::with_capacity(param_names.len());

        for param in &param_names {
            params.push(self.locals.declare(param));
        }

        let body = self.resolve_expr(body, ExprArea::FunctionBody)?;
        self.locals.end_function();
        Ok(hir::Expr::Function(params, body.into()))
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
        Ok(hir::Expr::Unary(op, rhs.into()))
    }

    /// Resolves a binary [`Expr`] to an [`hir::Expr`]. This function returns a
    /// [`ResolveError`] if either operand is a statement or could not be
    /// resolved.
    fn resolve_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> Result<hir::Expr> {
        let lhs = self.resolve_expr(lhs, ExprArea::Operand)?;
        let rhs = self.resolve_expr(rhs, ExprArea::Operand)?;
        Ok(hir::Expr::Binary(op, lhs.into(), rhs.into()))
    }

    /// Returns `true` if a global variable is defined.
    fn is_global_defined(&self, name: &str) -> bool {
        self.globals.is_defined(name) || self.new_globals.contains(name)
    }

    /// Returns an [`hir::Stmt`] defining a global variable with a name and a
    /// value. This function returns a [`ResolveError`] if a global variable is
    /// already defined with the given name.
    fn define_global(&mut self, name: &str, value: hir::Expr) -> Result<hir::Stmt> {
        if self.is_global_defined(name) {
            return Err(ResolveError::AlreadyDefinedVariable(name.to_owned()));
        }

        self.new_globals.insert(name.to_owned());
        Ok(hir::Stmt::AssignGlobal(name.to_owned(), value.into()))
    }

    /// Returns an [`hir::Stmt`] defining a local variable with a name and a
    /// value. This function returns a [`ResolveError`] if a local variable is
    /// already defined with the given name in the current scope.
    fn define_local(&mut self, name: &str, value: hir::Expr) -> Result<hir::Stmt> {
        if self.locals.contains_inner(name) {
            return Err(ResolveError::AlreadyDefinedVariable(name.to_owned()));
        }

        let id = self.locals.declare(name);
        Ok(hir::Stmt::DefineLocal(id, value.into()))
    }
}

/// A kind of `Scope` where a variable may defined.
#[derive(Clone, Copy)]
pub enum ScopeKind {
    /// At the top level of the program.
    Global,

    /// Inside a block or function parameter.
    Local,
}
