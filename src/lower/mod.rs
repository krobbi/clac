mod errors;
mod scopes;

use std::slice;

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
        let stmts = self.lower_sequence(&ast.0);
        Hir(stmts.into_boxed_slice())
    }

    /// Lowers a sequence of statement [`Expr`]s to a sequence of
    /// [`hir::Stmt`]s.
    fn lower_sequence(&mut self, stmts: &[Expr]) -> Vec<hir::Stmt> {
        let mut lowered_stmts = Vec::with_capacity(stmts.len());

        for stmt in stmts {
            let stmt = self.lower_stmt(stmt);
            lowered_stmts.push(stmt);
        }

        lowered_stmts
    }

    /// Lowers a statement [`Expr`] to an [`hir::Stmt`].
    fn lower_stmt(&mut self, stmt: &Expr) -> hir::Stmt {
        match self.lower_node(stmt) {
            Node::Stmt(stmt) => stmt,
            Node::Expr(expr) => {
                if self.scopes.is_global_scope() {
                    hir::Stmt::Print(Box::new(expr))
                } else {
                    hir::Stmt::Expr(Box::new(expr))
                }
            }
        }
    }

    /// Lowers an [`Expr`] to an [`hir::Expr`] in an [`ExprArea`].
    fn lower_expr(&mut self, expr: &Expr, area: ExprArea) -> hir::Expr {
        match self.lower_node(expr) {
            Node::Stmt(_) => self.error_expr(ErrorKind::UsedStmt(area)),
            Node::Expr(expr) => expr,
        }
    }

    /// Lowers an [`Expr`] to a [`Node`].
    fn lower_node(&mut self, expr: &Expr) -> Node {
        let expr = match expr {
            Expr::Literal(literal) => hir::Expr::Literal(*literal),
            Expr::Variable(symbol) => self.lower_expr_variable(*symbol),
            Expr::Paren(expr) => self.lower_expr(expr, ExprArea::Paren),
            Expr::Tuple(_) => self.error_expr(ErrorKind::TupleValue),
            Expr::Block(stmts) => return self.lower_expr_block(stmts),
            Expr::Assign(target, source) => return self.lower_expr_assign(target, source).into(),
            Expr::Function(list, body) => self.lower_expr_function(None, list, body),
            Expr::Call(callee, list) => self.lower_expr_call(callee, list),
            Expr::Unary(op, rhs) => self.lower_expr_unary(*op, rhs),
            Expr::Binary(op, lhs, rhs) => self.lower_expr_binary(*op, lhs, rhs),
            Expr::Logic(op, lhs, rhs) => self.lower_expr_logic(*op, lhs, rhs),
            Expr::Cond(cond, then, or) => self.lower_expr_cond(cond, then, or),
        };

        expr.into()
    }

    /// Lowers a variable [`Expr`] to an [`hir::Expr`].
    fn lower_expr_variable(&mut self, symbol: Symbol) -> hir::Expr {
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
            None => hir::Stmt::Block(Box::new([])).into(),
            Some(hir::Stmt::Expr(expr)) => hir::Expr::Block(stmts.into_boxed_slice(), expr).into(),
            Some(stmt) => {
                stmts.push(stmt);
                hir::Stmt::Block(stmts.into_boxed_slice()).into()
            }
        }
    }

    /// Lowers an assignment [`Expr`] to an [`hir::Stmt`].
    fn lower_expr_assign(&mut self, target: &Expr, source: &Expr) -> hir::Stmt {
        let (symbol, value) = match target {
            Expr::Variable(symbol) => {
                let value = self.lower_expr(source, ExprArea::AssignSource);
                (*symbol, value)
            }
            Expr::Call(callee, list) => {
                let Expr::Variable(symbol) = callee.as_ref() else {
                    return self.error_stmt(ErrorKind::InvalidFunctionName);
                };

                let symbol = *symbol;
                let value = self.lower_expr_function(Some(symbol), list, source);
                (symbol, value)
            }
            _ => return self.error_stmt(ErrorKind::InvalidAssignTarget),
        };

        match self.scopes.declare_variable(symbol) {
            None => self.error_stmt(ErrorKind::AlreadyDefinedVariable(symbol)),
            Some(Variable::Global) => hir::Stmt::AssignGlobal(symbol, Box::new(value)),
            Some(Variable::Local(local)) => hir::Stmt::DefineLocal(local, Box::new(value)),
        }
    }

    /// Lowers a function [`Expr`] to an [`hir::Expr`].
    fn lower_expr_function(&mut self, name: Option<Symbol>, list: &Expr, body: &Expr) -> hir::Expr {
        self.scopes.push_function_scope();

        let name = name.map(|s| {
            let Some(Variable::Local(local)) = self.scopes.declare_variable(s) else {
                unreachable!("there should be an empty function scope");
            };

            local
        });

        self.scopes.push_param_scope();
        let params = slice_list(list);
        let mut lowered_params = Vec::with_capacity(params.len());

        for param in params {
            let Expr::Variable(symbol) = param else {
                self.scopes.pop_param_scope();
                self.scopes.pop_function_scope();
                return self.error_expr(ErrorKind::InvalidParam);
            };

            let Some(Variable::Local(local)) = self.scopes.declare_variable(*symbol) else {
                self.scopes.pop_param_scope();
                self.scopes.pop_function_scope();
                return self.error_expr(ErrorKind::DuplicateParam(*symbol));
            };

            lowered_params.push(local);
        }

        let body = self.lower_expr(body, ExprArea::FunctionBody);
        self.scopes.pop_param_scope();
        self.scopes.pop_function_scope();
        hir::Expr::Function(name, lowered_params.into_boxed_slice(), Box::new(body))
    }

    /// Lowers a function call [`Expr`] to an [`hir::Expr`].
    fn lower_expr_call(&mut self, callee: &Expr, list: &Expr) -> hir::Expr {
        let callee = self.lower_expr(callee, ExprArea::Callee);
        let args = slice_list(list);
        let mut lowered_args = Vec::with_capacity(args.len());

        for arg in args {
            let arg = self.lower_expr(arg, ExprArea::Arg);
            lowered_args.push(arg);
        }

        hir::Expr::Call(Box::new(callee), lowered_args.into_boxed_slice())
    }

    /// Lowers a unary [`Expr`] to an [`hir::Expr`].
    fn lower_expr_unary(&mut self, op: UnOp, rhs: &Expr) -> hir::Expr {
        let rhs = self.lower_expr(rhs, ExprArea::Operand);
        hir::Expr::Unary(op, Box::new(rhs))
    }

    /// Lowers a binary [`Expr`] to an [`hir::Expr`].
    fn lower_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let lhs = self.lower_expr(lhs, ExprArea::Operand);
        let rhs = self.lower_expr(rhs, ExprArea::Operand);
        hir::Expr::Binary(op, Box::new(lhs), Box::new(rhs))
    }

    /// Lowers a short-circuiting logical [`Expr`] to an [`hir::Expr`].
    fn lower_expr_logic(&mut self, op: LogicOp, lhs: &Expr, rhs: &Expr) -> hir::Expr {
        let lhs = self.lower_expr(lhs, ExprArea::Operand);
        let rhs = self.lower_expr(rhs, ExprArea::Operand);

        // HACK: Dynamic type check for right-hand side.
        let rhs = hir::Expr::Binary(
            BinOp::Equal,
            Box::new(rhs),
            Box::new(hir::Expr::Literal(Literal::Bool(true))),
        );

        let (then_expr, else_expr) = match op {
            LogicOp::And => (rhs, hir::Expr::Literal(Literal::Bool(false))),
            LogicOp::Or => (hir::Expr::Literal(Literal::Bool(true)), rhs),
        };

        hir::Expr::Cond(Box::new(lhs), Box::new(then_expr), Box::new(else_expr))
    }

    /// Lowers a ternary conditional [`Expr`] to an [`hir::Expr`].
    fn lower_expr_cond(&mut self, cond: &Expr, then_expr: &Expr, else_expr: &Expr) -> hir::Expr {
        let cond = self.lower_expr(cond, ExprArea::Condition);
        let then_expr = self.lower_expr(then_expr, ExprArea::Operand);
        let else_expr = self.lower_expr(else_expr, ExprArea::Operand);
        hir::Expr::Cond(Box::new(cond), Box::new(then_expr), Box::new(else_expr))
    }

    /// Reports an [`ErrorKind`] and creates a new synthetic [`hir::Stmt`] for
    /// error recovery.
    fn error_stmt(&mut self, error: ErrorKind) -> hir::Stmt {
        self.report_error(error);
        hir::Stmt::Block(Box::new([]))
    }

    /// Reports an [`ErrorKind`] and creates a new synthetic [`hir::Expr`] for
    /// error recovery.
    fn error_expr(&mut self, error: ErrorKind) -> hir::Expr {
        self.report_error(error);
        hir::Expr::Literal(Literal::Number(0.0))
    }

    /// Reports an [`ErrorKind`].
    #[cold]
    fn report_error(&mut self, error: ErrorKind) {
        self.error
            .get_or_insert_with(|| LowerError(Box::new(error)));
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

/// Returns a function parameter or call argument list [`Expr`] as a slice of
/// parameter or argument [`Expr`]s.
const fn slice_list(list: &Expr) -> &[Expr] {
    match list {
        Expr::Paren(elem) => slice::from_ref(elem),
        Expr::Tuple(elems) => elems,
        elem => slice::from_ref(elem),
    }
}
