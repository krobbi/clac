mod errors;
mod nodes;
mod scopes;

use thiserror::Error;

use crate::{
    ast::{Ast, BinOp, Expr, Literal, LogicOp, UnOp},
    interpreter::Globals,
    symbols::Symbol,
};

use super::{
    ir::{self, Ir},
    locals::LocalTable,
};

use self::{
    errors::{ErrorKind, ExprArea},
    nodes::Node,
    scopes::{ScopeStack, Variable},
};

/// An error caught while lowering an [`Ast`] to [`Ir`].
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct LoweringError(Box<ErrorKind>);

/// Lower an [`Ast`] to [`Ir`] with [`Globals`] and a [`LocalTable`]. This
/// function returns a [`LoweringError`] if the [`Ast`] could not be lowered.
pub fn lower_ast(
    ast: &Ast,
    globals: &Globals,
    locals: &mut LocalTable,
) -> Result<Ir, LoweringError> {
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

/// A structure which lowers an [`Ast`] to [`Ir`].
struct Lowerer<'loc> {
    /// The [`ScopeStack`].
    scopes: ScopeStack<'loc>,

    /// The first [`LoweringError`], if any.
    error: Option<LoweringError>,
}

impl<'loc> Lowerer<'loc> {
    /// Creates a new `Lowerer` from a [`ScopeStack`].
    const fn new(scopes: ScopeStack<'loc>) -> Self {
        Self {
            scopes,
            error: None,
        }
    }

    /// Lowers an [`Ast`] to [`Ir`].
    fn lower_ast(&mut self, ast: &Ast) -> Ir {
        Ir(self.lower_sequence(&ast.0).into())
    }

    /// Lowers a sequence of statement [`Expr`]s to a sequence of [`ir::Stmt`]s.
    fn lower_sequence(&mut self, stmts: &[Expr]) -> Vec<ir::Stmt> {
        let mut lowered_stmts = Vec::with_capacity(stmts.len());

        for stmt in stmts {
            lowered_stmts.push(self.lower_stmt(stmt));
        }

        lowered_stmts
    }

    /// Lowers a statement [`Expr`] to an [`ir::Stmt`].
    fn lower_stmt(&mut self, stmt: &Expr) -> ir::Stmt {
        match self.lower_node(stmt) {
            Node::Stmt(stmt) => stmt,
            Node::Expr(expr) => {
                if self.scopes.is_global_scope() {
                    ir::Stmt::Print(expr.into())
                } else {
                    ir::Stmt::Expr(expr.into())
                }
            }
        }
    }

    /// Lowers an [`Expr`] to an [`ir::Expr`] in an [`ExprArea`].
    fn lower_expr(&mut self, expr: &Expr, area: ExprArea) -> ir::Expr {
        match self.lower_node(expr) {
            Node::Stmt(_) => self.error_expr(area.into()),
            Node::Expr(expr) => expr,
        }
    }

    /// Lowers an [`Expr`] to a [`Node`].
    fn lower_node(&mut self, expr: &Expr) -> Node {
        let expr = match expr {
            Expr::Literal(literal) => ir::Expr::Literal(literal.clone()),
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

    /// Lowers an identifier [`Expr`] to an [`ir::Expr`].
    fn lower_expr_ident(&mut self, symbol: Symbol) -> ir::Expr {
        match self.scopes.variable(symbol) {
            None => self.error_expr(ErrorKind::UndefinedVariable(symbol)),
            Some(Variable::Global) => ir::Expr::Global(symbol),
            Some(Variable::Local(local)) => ir::Expr::Local(local),
        }
    }

    /// Lowers a block [`Expr`] to a [`Node`].
    fn lower_expr_block(&mut self, stmts: &[Expr]) -> Node {
        self.scopes.push_block_scope();
        let mut stmts = self.lower_sequence(stmts);
        self.scopes.pop_block_scope();

        match stmts.pop() {
            None => ir::Stmt::Block([].into()).into(),
            Some(ir::Stmt::Expr(expr)) => ir::Expr::Block(stmts.into(), expr).into(),
            Some(stmt) => {
                stmts.push(stmt);
                ir::Stmt::Block(stmts.into()).into()
            }
        }
    }

    /// Lowers an assignment [`Expr`] to an [`ir::Stmt`].
    fn lower_expr_assign(&mut self, target: &Expr, source: &Expr) -> ir::Stmt {
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
            Some(Variable::Global) => ir::Stmt::AssignGlobal(symbol, value.into()),
            Some(Variable::Local(local)) => ir::Stmt::DefineLocal(local, value.into()),
        }
    }

    /// Lowers a function [`Expr`] to an [`ir::Expr`].
    fn lower_expr_function(&mut self, params: &[Expr], body: &Expr) -> ir::Expr {
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
        ir::Expr::Function(lowered_params.into(), body.into())
    }

    /// Lowers a function call [`Expr`] to an [`ir::Expr`].
    fn lower_expr_call(&mut self, callee: &Expr, args: &[Expr]) -> ir::Expr {
        let callee = self.lower_expr(callee, ExprArea::Callee);
        let mut lowered_args = Vec::with_capacity(args.len());

        for arg in args {
            lowered_args.push(self.lower_expr(arg, ExprArea::Arg));
        }

        ir::Expr::Call(callee.into(), lowered_args.into())
    }

    /// Lowers a unary [`Expr`] to an [`ir::Expr`].
    fn lower_expr_unary(&mut self, op: UnOp, rhs: &Expr) -> ir::Expr {
        let rhs = self.lower_expr(rhs, ExprArea::Operand);
        ir::Expr::Unary(op, rhs.into())
    }

    /// Lowers a binary [`Expr`] to an [`ir::Expr`].
    fn lower_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) -> ir::Expr {
        let lhs = self.lower_expr(lhs, ExprArea::Operand);
        let rhs = self.lower_expr(rhs, ExprArea::Operand);
        ir::Expr::Binary(op, lhs.into(), rhs.into())
    }

    /// Lowers a short-circuiting logical [`Expr`] to an [`ir::Expr`].
    fn lower_expr_logic(&mut self, op: LogicOp, lhs: &Expr, rhs: &Expr) -> ir::Expr {
        let lhs = self.lower_expr(lhs, ExprArea::Operand);
        let rhs = self.lower_expr(rhs, ExprArea::Operand);

        // Compare the right-hand side with [`true`] for dynamic type checking.
        let rhs = ir::Expr::Binary(
            BinOp::Equal,
            rhs.into(),
            ir::Expr::Literal(Literal::Bool(true)).into(),
        );

        let (then_expr, else_expr) = match op {
            LogicOp::And => (rhs, ir::Expr::Literal(Literal::Bool(false))),
            LogicOp::Or => (ir::Expr::Literal(Literal::Bool(true)), rhs),
        };

        ir::Expr::Cond(lhs.into(), then_expr.into(), else_expr.into())
    }

    /// Lowers a ternary conditional [`Expr`] to an [`ir::Expr`].
    fn lower_expr_cond(&mut self, cond: &Expr, then_expr: &Expr, else_expr: &Expr) -> ir::Expr {
        let cond = self.lower_expr(cond, ExprArea::Condition);
        let then_expr = self.lower_expr(then_expr, ExprArea::Operand);
        let else_expr = self.lower_expr(else_expr, ExprArea::Operand);
        ir::Expr::Cond(cond.into(), then_expr.into(), else_expr.into())
    }

    /// Reports an [`ErrorKind`] and returns a default [`ir::Stmt`].
    #[cold]
    fn error_stmt(&mut self, error: ErrorKind) -> ir::Stmt {
        self.report_error(error);
        ir::Stmt::Block([].into())
    }

    /// Reports an [`ErrorKind`] and returns a default [`ir::Expr`].
    #[cold]
    fn error_expr(&mut self, error: ErrorKind) -> ir::Expr {
        self.report_error(error);
        ir::Expr::Literal(Literal::Number(0.0))
    }

    /// Reports an [`ErrorKind`].
    #[cold]
    fn report_error(&mut self, error: ErrorKind) {
        self.error
            .get_or_insert_with(|| LoweringError(error.into()));
    }
}
