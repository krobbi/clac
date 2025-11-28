mod body;
mod upvalue_table;

use std::mem;

use crate::{
    ast::{BinOp, Literal},
    cfg::{Block, Cfg, Exit, Instruction, Label},
    decl_table::{DeclId, DeclTable},
    hir::{Expr, Hir, Stmt},
    ir::{self, Function, Ir, Value},
};

use self::{body::Body, upvalue_table::UpvalueTable};

/// Compiles [`Hir`] to [`Ir`] and a [`Cfg`] with a [`DeclTable`].
pub fn compile_hir(hir: &Hir, decls: &DeclTable) -> (Ir, Cfg) {
    let mut cfg = Cfg::new();
    let mut compiler = Compiler::new(decls, &mut cfg);
    compiler.compile_hir(hir);
    (Ir(compiler.into_body()), cfg)
}

/// A structure that compiles [`Hir`] to an [`ir::Body`] and a [`Cfg`].
struct Compiler<'a, 'b> {
    /// The [`DeclTable`].
    decls: &'a DeclTable,

    /// The [`UpvalueTable`].
    upvalues: UpvalueTable,

    /// The current call depth.
    call_depth: usize,

    /// The current [`Body`].
    body: Body,

    /// The current [`Label`].
    label: Label,

    /// The [`Cfg`].
    cfg: &'b mut Cfg,
}

impl<'a, 'b> Compiler<'a, 'b> {
    /// Creates a new `Compiler` from a [`DeclTable`] and a [`Cfg`].
    fn new(decls: &'a DeclTable, cfg: &'b mut Cfg) -> Self {
        Self {
            decls,
            upvalues: UpvalueTable::new(),
            call_depth: 0,
            body: Body::new(0),
            label: Label::default(),
            cfg,
        }
    }

    /// Consumes the `Compiler` and converts it to an [`ir::Body`].
    fn into_body(self) -> ir::Body {
        self.body.into_body()
    }

    /// Compiles [`Hir`].
    fn compile_hir(&mut self, hir: &Hir) {
        self.compile_stmts(&hir.0);
    }

    /// Compiles a slice of [`Stmt`]s.
    fn compile_stmts(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
    }

    /// Compiles a [`Stmt`].
    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Nop => (),
            Stmt::Block(stmts) => self.compile_stmt_block(stmts),
            Stmt::AssignGlobal(name, value) => self.compile_stmt_assign_global(name, value),
            Stmt::DefineLocal(id, value) => self.compile_stmt_define_local(*id, value),
            Stmt::Print(value) => self.compile_stmt_print(value),
            Stmt::Expr(expr) => self.compile_stmt_expr(expr),
        }
    }

    /// Compiles a block [`Stmt`].
    fn compile_stmt_block(&mut self, stmts: &[Stmt]) {
        self.body.stack.begin_scope();
        self.compile_stmts(stmts);
        let local_count = self.body.stack.end_scope();
        self.compile_drop(local_count);
    }

    /// Compiles a global variable assignment [`Stmt`].
    fn compile_stmt_assign_global(&mut self, name: &str, value: &Expr) {
        self.compile_expr(value);
        self.compile(Instruction::StoreGlobal(name.to_owned()));
        self.compile_ir(ir::Instruction::StoreGlobal(name.to_owned()));
    }

    /// Compiles a local variable definition [`Stmt`].
    fn compile_stmt_define_local(&mut self, id: DeclId, value: &Expr) {
        self.compile_expr(value);

        if self.decls.get(id).is_upvalue {
            self.compile_define_upvalue(id);
        } else {
            self.body.stack.declare_local(id);
        }
    }

    /// Compiles a print [`Stmt`].
    fn compile_stmt_print(&mut self, value: &Expr) {
        self.compile_expr(value);
        self.compile(Instruction::Print);
        self.compile_ir(ir::Instruction::Print);
    }

    /// Compiles an expression [`Stmt`].
    fn compile_stmt_expr(&mut self, expr: &Expr) {
        self.compile_expr(expr);
        self.compile(Instruction::Drop);
        self.compile_ir(ir::Instruction::Drop);
    }

    /// Compiles an [`Expr`].
    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(literal) => self.compile_expr_literal(literal),
            Expr::Global(name) => self.compile_expr_global(name),
            Expr::Local(id) => self.compile_expr_local(*id),
            Expr::Block(stmts, expr) => self.compile_expr_block(stmts, expr),
            Expr::Function(params, body) => self.compile_expr_function(params, body),
            Expr::Call(callee, args) => self.compile_expr_call(callee, args),
            Expr::Binary(op, lhs, rhs) => self.compile_expr_binary(*op, lhs, rhs),
        }
    }

    /// Compiles a literal [`Expr`].
    fn compile_expr_literal(&mut self, literal: &Literal) {
        self.compile(Instruction::PushLiteral(literal.clone()));

        match literal {
            Literal::Number(value) => self.compile_ir(ir::Instruction::Push(Value::Number(*value))),
        }
    }

    /// Compiles a global variable [`Expr`].
    fn compile_expr_global(&mut self, name: &str) {
        self.compile(Instruction::PushGlobal(name.to_owned()));
        self.compile_ir(ir::Instruction::LoadGlobal(name.to_owned()));
    }

    /// Compiles a local variable [`Expr`].
    fn compile_expr_local(&mut self, id: DeclId) {
        let decl = self.decls.get(id);

        if decl.is_upvalue {
            self.compile_ir(ir::Instruction::LoadUpvalue(id));
            self.body.access_upvalue(decl.call_depth);
        } else {
            self.compile_ir(ir::Instruction::LoadLocal(self.body.stack.local_offset(id)));
        }
    }

    /// Compiles a block [`Expr`].
    fn compile_expr_block(&mut self, stmts: &[Stmt], expr: &Expr) {
        self.body.stack.begin_scope();
        self.compile_stmts(stmts);
        self.compile_expr(expr);
        let local_count = self.body.stack.end_scope();

        if local_count > 0 {
            // The result of the block expression is on top of the stack, but
            // there are local variables below it that need to be dropped. Move
            // the result into the first local variable and drop any local
            // variables above it.
            self.compile_ir(ir::Instruction::StoreLocal(self.body.stack.len()));
            self.compile_drop(local_count - 1);
        }
    }

    /// Compiles a function [`Expr`].
    fn compile_expr_function(&mut self, params: &[DeclId], body: &Expr) {
        self.call_depth += 1;
        let outer_body = mem::replace(&mut self.body, Body::new(self.call_depth));
        let outer_label = mem::replace(&mut self.label, self.cfg.insert_block());

        // The function's arguments are already on the stack, but need to be
        // declared.
        for id in params.iter().copied() {
            if self.decls.get(id).is_upvalue {
                let offset = self.body.stack.len();
                self.body.stack.declare_intermediate();

                // Upvalue arguments are copied from the stack before they are
                // defined as upvalues. The caller has already placed all of the
                // arguments on the stack, so the top of the stack may not be
                // the upvalue that is expected. This load instruction could
                // possibly be eliminated for upvalues at the end of the
                // arguments list.
                self.compile_ir(ir::Instruction::LoadLocal(offset));
                self.compile_define_upvalue(id);
            } else {
                self.body.stack.declare_local(id);
            }
        }

        self.compile_expr(body);
        self.block_mut().exit = Exit::Return;
        self.label = outer_label;
        let upvalue_call_depth = self.body.upvalue_call_depth;

        let function = Function {
            arity: params.len(),
            body: mem::replace(&mut self.body, outer_body).into_body(),
        };

        self.call_depth -= 1;
        self.compile_ir(ir::Instruction::Push(Value::Function(function.into())));

        if upvalue_call_depth <= self.call_depth {
            // An upvalue accessed in the inner function may outlive the outer
            // function, so the outer function may need to be a closure.
            self.body.access_upvalue(upvalue_call_depth);

            // The inner function is outlived by an upvalue that it accesses, so
            // it must be converted to a closure.
            self.compile_ir(ir::Instruction::IntoClosure);
        }
    }

    /// Compiles a function call [`Expr`].
    fn compile_expr_call(&mut self, callee: &Expr, args: &[Expr]) {
        self.compile_expr(callee);
        self.body.stack.declare_intermediate();

        for arg in args {
            self.compile_expr(arg);
            self.body.stack.declare_intermediate();
        }

        let arity = args.len();
        self.compile_ir(ir::Instruction::Call(arity));
        self.body.stack.drop_intermediates(arity + 1);
    }

    /// Compiles a binary [`Expr`].
    fn compile_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) {
        self.compile_expr(lhs);
        self.body.stack.declare_intermediate();
        self.compile_expr(rhs);

        let op = match op {
            BinOp::Add => ir::BinOp::Add,
            BinOp::Subtract => ir::BinOp::Subtract,
            BinOp::Multiply => ir::BinOp::Multiply,
            BinOp::Divide => ir::BinOp::Divide,
        };

        self.compile_ir(ir::Instruction::Binary(op));
        self.body.stack.drop_intermediates(1);
    }

    /// Appends an [`Instruction`] to the current [`Block`].
    fn compile(&mut self, instruction: Instruction) {
        self.block_mut().instructions.push(instruction);
    }

    /// Appends an upvalue definition instruction to the current [`Block`].
    fn compile_define_upvalue(&mut self, id: DeclId) {
        self.compile_ir(ir::Instruction::DefineUpvalue(id));
        let id = self.upvalues.declare(id);
        self.compile(Instruction::DefineUpvalue(id));
    }

    /// Appends multiple drop [`Instruction`]s to the current [`Block`].
    fn compile_drop(&mut self, count: usize) {
        for _ in 0..count {
            self.compile(Instruction::Drop);
            self.compile_ir(ir::Instruction::Drop);
        }
    }

    /// Returns a mutable reference to the current [`Block`].
    fn block_mut(&mut self) -> &mut Block {
        self.cfg.block_mut(self.label)
    }

    /// Compiles an [`ir::Instruction`] for the current [`ir::Body`]. This
    /// function is deprecated and will be removed when the [`Cfg`] is
    /// implemented.
    fn compile_ir(&mut self, instruction: ir::Instruction) {
        self.body.instructions.push(instruction);
    }
}
