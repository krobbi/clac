mod errors;
mod scopes;

use thiserror::Error;

use crate::{
    ast::{Ast, BinOp, Expr, Literal, LogicOp, UnOp},
    hir::{self, Hir},
    interpret::Globals,
    locals::LocalTable,
    symbols::Symbol,
};

use self::{
    errors::{ErrorKind, ExprArea},
    scopes::{ScopeStack, Variable},
};

/// An error caught while lowering an [`Ast`].
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct LowerError(Box<ErrorKind>);

/// Lower an [`Ast`] to [`Hir`] with [`Globals`] and a [`LocalTable`]. This
/// function returns a [`LowerError`] if the [`Ast`] could not be lowered.
pub fn lower_ast(ast: &Ast, globals: &Globals, locals: &mut LocalTable) -> Result<Hir, LowerError> {
    let mut scopes = ScopeStack::new(locals);

    for symbol in globals.symbols() {
        let variable = scopes.declare_variable(symbol);

        debug_assert!(
            matches!(variable, Some(Variable::Global)),
            "declared variables should all be globals"
        );
    }

    let mut lowerer = Lowerer::new(scopes);
    let ir = lowerer.lower_ast(ast);

    debug_assert!(
        lowerer.scopes.is_global_scope(),
        "scope stack should be empty after lowering"
    );

    lowerer.error.map_or(Ok(ir), Err)
}

/// A structure which lowers an [`Ast`] to [`Hir`].
struct Lowerer<'loc> {
    /// The [`ScopeStack`].
    scopes: ScopeStack<'loc>,

    /// The first [`LowerError`], if any.
    error: Option<LowerError>,
}

impl<'loc> Lowerer<'loc> {
    /// Creates a new `Lowerer` from a [`ScopeStack`].
    const fn new(scopes: ScopeStack<'loc>) -> Self {
        Self {
            scopes,
            error: None,
        }
    }

    /// Lowers an [`Ast`] to [`Hir`].
    fn lower_ast(&mut self, ast: &Ast) -> Hir {
        Hir(self.lower_sequence(&ast.0).into())
    }

    /// Lowers a sequence of statement [`Expr`]s to a sequence of
    /// [`hir::Stmt`]s.
    fn lower_sequence(&mut self, stmts: &[Expr]) -> Vec<hir::Stmt> {
        let mut lowered_stmts = Vec::with_capacity(stmts.len());

        for stmt in stmts {
            lowered_stmts.push(self.lower_stmt(stmt));
        }

        lowered_stmts
    }

    /// Lowers a statement [`Expr`] to an [`hir::Stmt`].
    fn lower_stmt(&mut self, stmt: &Expr) -> hir::Stmt {
        match self.lower_node(stmt) {
            Node::Stmt(stmt) => stmt,
            Node::Expr(expr) => {
                if self.scopes.is_global_scope() {
                    hir::Stmt::Print(expr.into())
                } else {
                    hir::Stmt::Expr(expr.into())
                }
            }
        }
    }

    /// Lowers an [`Expr`] to an [`hir::Expr`] in an [`ExprArea`].
    fn lower_expr(&mut self, expr: &Expr, area: ExprArea) -> hir::Expr {
        match self.lower_node(expr) {
            Node::Stmt(_) => self.error_expr(area.into()),
            Node::Expr(expr) => expr,
        }
    }

    /// Lowers an [`Expr`] to a [`Node`].
    fn lower_node(&mut self, expr: &Expr) -> Node {
        let expr = match expr {
            Expr::Literal(literal) => hir::Expr::Literal(*literal),
            Expr::Ident(symbol) => self.lower_expr_ident(*symbol),
            Expr::Paren(expr) => self.lower_expr(expr, ExprArea::Paren),
            Expr::Tuple(_) => self.error_expr(ErrorKind::TupleValue),
            Expr::Block(stmts) => return self.lower_expr_block(stmts),
            Expr::Assign(target, source) => return self.lower_expr_assign(target, source).into(),
            Expr::Function(params, body) => self.lower_expr_function(params, body),
            Expr::Call(callee, args) => self.lower_expr_call(callee, args),
            Expr::Unary(op, rhs) => self.lower_expr_unary(*op, rhs),
            Expr::Binary(op, lhs, rhs) => self.lower_expr_binary(*op, lhs, rhs),
            Expr::Logic(op, lhs, rhs) => self.lower_expr_logic(*op, lhs, rhs),
            Expr::Cond(cond, then, or) => self.lower_expr_cond(cond, then, or),
        };

        expr.into()
    }

    /// Lowers an identifier [`Expr`] to an [`hir::Expr`].
    fn lower_expr_ident(&mut self, symbol: Symbol) -> hir::Expr {
        match self.scopes.variable(symbol) {
            None => self.error_expr(ErrorKind::UndefinedVariable(symbol)),
            Some(Variable::Global) => hir::Expr::Global(symbol),
            Some(Variable::Local(local)) => hir::Expr::Local(local),
        }
    }

    /// Lowers a block [`Expr`] to a [`Node`].
    fn lower_expr_block(&mut self, stmts: &[Expr]) -> Node {
        self.scopes.push_block_scope();
        let mut stmts = self.lower_sequence(stmts);
        self.scopes.pop_block_scope();

        match stmts.pop() {
            None => hir::Stmt::Block([].into()).into(),
            Some(hir::Stmt::Expr(expr)) => hir::Expr::Block(stmts.into(), expr).into(),
            Some(stmt) => {
                stmts.push(stmt);
                hir::Stmt::Block(stmts.into()).into()
            }
        }
    }

    /// Lowers an assignment [`Expr`] to an [`hir::Stmt`].
    fn lower_expr_assign(&mut self, target: &Expr, source: &Expr) -> hir::Stmt {
        let (symbol, value) = match target {
            Expr::Ident(symbol) => (*symbol, self.lower_expr(source, ExprArea::AssignSource)),
            Expr::Call(callee, args) => {
                let Expr::Ident(symbol) = callee.as_ref() else {
                    return self.error_stmt(ErrorKind::InvalidFunctionName);
                };

                (*symbol, self.lower_expr_function(args, source))
            }
            _ => return self.error_stmt(ErrorKind::InvalidAssignTarget),
        };

        match self.scopes.declare_variable(symbol) {
            None => self.error_stmt(ErrorKind::AlreadyDefinedVariable(symbol)),
            Some(Variable::Global) => hir::Stmt::AssignGlobal(symbol, value.into()),
            Some(Variable::Local(local)) => hir::Stmt::DefineLocal(local, value.into()),
        }
    }

    /// Lowers a function [`Expr`] to an [`hir::Expr`].
    fn lower_expr_function(&mut self, params: &[Expr], body: &Expr) -> hir::Expr {
        self.scopes.push_function_scope();
        let mut lowered_params = Vec::with_capacity(params.len());

        for param in params {
            let Expr::Ident(symbol) = param else {
                self.scopes.pop_function_scope();
                return self.error_expr(ErrorKind::InvalidParam);
            };

            let Some(Variable::Local(local)) = self.scopes.declare_variable(*symbol) else {
                self.scopes.pop_function_scope();
                return self.error_expr(ErrorKind::DuplicateParam(*symbol));
            };

            lowered_params.push(local);
        }

        let body = self.lower_expr(body, ExprArea::FunctionBody);
        self.scopes.pop_function_scope();
        hir::Expr::Function(lowered_params.into(), body.into())
    }

    /// Lowers a function call [`Expr`] to an [`hir::Expr`].
    fn lower_expr_call(&mut self, callee: &Expr, args: &[Expr]) -> hir::Expr {
        let callee = self.lower_expr(callee, ExprArea::Callee);
        let mut lowered_args = Vec::with_capacity(args.len());

        for arg in args {
            lowered_args.push(self.lower_expr(arg, ExprArea::Arg));
        }

        hir::Expr::Call(callee.into(), lowered_args.into())
    }

    /// Lowers a unary [`Expr`] to an [`hir::Expr`].
    fn lower_expr_unary(&mut self, op: UnOp, rhs: &Expr) -> hir::Expr {
        let rhs = self.lower_expr(rhs, ExprArea::Operand);
        hir::Expr::Unary(op, rhs.into())
    }

    /// Lowers a binary [`Expr`] to an [`hir::Expr`].
    fn lower_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let lhs = self.lower_expr(lhs, ExprArea::Operand);
        let rhs = self.lower_expr(rhs, ExprArea::Operand);
        hir::Expr::Binary(op, lhs.into(), rhs.into())
    }

    /// Lowers a short-circuiting logical [`Expr`] to an [`hir::Expr`].
    fn lower_expr_logic(&mut self, op: LogicOp, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let lhs = self.lower_expr(lhs, ExprArea::Operand);
        let rhs = self.lower_expr(rhs, ExprArea::Operand);

        // Compare the right-hand side with [`true`] for dynamic type checking.
        let rhs = hir::Expr::Binary(
            BinOp::Equal,
            rhs.into(),
            hir::Expr::Literal(Literal::Bool(true)).into(),
        );

        let (then_expr, else_expr) = match op {
            LogicOp::And => (rhs, hir::Expr::Literal(Literal::Bool(false))),
            LogicOp::Or => (hir::Expr::Literal(Literal::Bool(true)), rhs),
        };

        hir::Expr::Cond(lhs.into(), then_expr.into(), else_expr.into())
    }

    /// Lowers a ternary conditional [`Expr`] to an [`hir::Expr`].
    fn lower_expr_cond(&mut self, cond: &Expr, then_expr: &Expr, else_expr: &Expr) -> hir::Expr {
        let cond = self.lower_expr(cond, ExprArea::Condition);
        let then_expr = self.lower_expr(then_expr, ExprArea::Operand);
        let else_expr = self.lower_expr(else_expr, ExprArea::Operand);
        hir::Expr::Cond(cond.into(), then_expr.into(), else_expr.into())
    }

    /// Reports an [`ErrorKind`] and returns a default [`hir::Stmt`].
    #[cold]
    fn error_stmt(&mut self, error: ErrorKind) -> hir::Stmt {
        self.report_error(error);
        hir::Stmt::Block([].into())
    }

    /// Reports an [`ErrorKind`] and returns a default [`hir::Expr`].
    #[cold]
    fn error_expr(&mut self, error: ErrorKind) -> hir::Expr {
        self.report_error(error);
        hir::Expr::Literal(Literal::Number(0.0))
    }

    /// Reports an [`ErrorKind`].
    #[cold]
    fn report_error(&mut self, error: ErrorKind) {
        self.error.get_or_insert_with(|| LowerError(error.into()));
    }
}

/// An [`Hir`] node which is either an [`hir::Stmt`] or an [`hir::Expr`].
enum Node {
    /// An [`hir::Stmt`].
    Stmt(hir::Stmt),

    /// An [`hir::Expr`].
    Expr(hir::Expr),
}

impl From<hir::Stmt> for Node {
    fn from(value: hir::Stmt) -> Self {
        Self::Stmt(value)
    }
}

impl From<hir::Expr> for Node {
    fn from(value: hir::Expr) -> Self {
        Self::Expr(value)
    }
}
