mod body;
mod upvalue_table;

use std::mem;

use crate::{
    ast::{BinOp, Literal},
    cfg::{Block, Cfg, Exit, Instruction, Label},
    decl_table::{DeclId, DeclTable},
    hir::{Expr, Hir, Stmt},
};

use self::{body::Body, upvalue_table::UpvalueTable};

/// Compiles [`Hir`] to a [`Cfg`] with a [`DeclTable`].
pub fn compile_hir(hir: &Hir, decls: &DeclTable) -> Cfg {
    let mut cfg = Cfg::new();
    let mut compiler = Compiler::new(decls, &mut cfg);
    compiler.compile_hir(hir);
    cfg
}

/// A structure that compiles [`Hir`] to a [`Cfg`].
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
    }

    /// Compiles an expression [`Stmt`].
    fn compile_stmt_expr(&mut self, expr: &Expr) {
        self.compile_expr(expr);
        self.compile(Instruction::Drop);
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
    }

    /// Compiles a global variable [`Expr`].
    fn compile_expr_global(&mut self, name: &str) {
        self.compile(Instruction::PushGlobal(name.to_owned()));
    }

    /// Compiles a local variable [`Expr`].
    fn compile_expr_local(&mut self, id: DeclId) {
        let decl = self.decls.get(id);

        if decl.is_upvalue {
            let id = self.upvalues.get(id);
            self.compile(Instruction::PushUpvalue(id));
            self.body.access_upvalue(decl.call_depth);
        } else {
            let offset = self.body.stack.local_offset(id);
            self.compile(Instruction::PushLocal(offset));
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
            let offset = self.body.stack.len();
            self.compile(Instruction::StoreLocal(offset));
            self.compile_drop(local_count - 1);
        }
    }

    /// Compiles a function [`Expr`].
    fn compile_expr_function(&mut self, params: &[DeclId], body: &Expr) {
        self.call_depth += 1;
        let outer_body = mem::replace(&mut self.body, Body::new(self.call_depth));

        let function_label = self.cfg.insert_block();
        let outer_label = self.label;
        self.label = function_label;

        // The function's arguments are already on the stack, but need to be
        // declared.
        for id in params.iter().copied() {
            if self.decls.get(id).is_upvalue {
                let offset = self.body.stack.len();
                self.body.stack.declare_intermediate();

                // Upvalue arguments are copied from the stack before they are
                // defined as upvalues. The caller has already placed all of the
                // arguments on the stack, so the top of the stack may not be
                // the upvalue that is expected.
                self.compile(Instruction::PushLocal(offset));
                self.compile_define_upvalue(id);
            } else {
                self.body.stack.declare_local(id);
            }
        }

        self.compile_expr(body);
        self.block_mut().exit = Exit::Return;
        self.label = outer_label;

        let upvalue_call_depth = self.body.upvalue_call_depth;
        self.body = outer_body;
        self.call_depth -= 1;

        self.compile(Instruction::PushFunction(function_label, params.len()));

        if upvalue_call_depth <= self.call_depth {
            // An upvalue accessed in the inner function may outlive the outer
            // function, so the outer function may need to be a closure.
            self.body.access_upvalue(upvalue_call_depth);

            // The inner function is outlived by an upvalue that it accesses, so
            // it must be converted to a closure.
            self.compile(Instruction::IntoClosure);
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
        let return_label = self.compile_branch_target();
        self.block_mut().exit = Exit::Call(arity, return_label);
        self.label = return_label;
        self.body.stack.drop_intermediates(arity + 1);
    }

    /// Compiles a binary [`Expr`].
    fn compile_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) {
        self.compile_expr(lhs);
        self.body.stack.declare_intermediate();
        self.compile_expr(rhs);
        self.compile(Instruction::Binary(op));
        self.body.stack.drop_intermediates(1);
    }

    /// Appends an [`Instruction`] to the current [`Block`].
    fn compile(&mut self, instruction: Instruction) {
        self.block_mut().instructions.push(instruction);
    }

    /// Appends an upvalue definition instruction to the current [`Block`].
    fn compile_define_upvalue(&mut self, id: DeclId) {
        let id = self.upvalues.declare(id);
        self.compile(Instruction::DefineUpvalue(id));
    }

    /// Appends multiple drop [`Instruction`]s to the current [`Block`].
    fn compile_drop(&mut self, count: usize) {
        for _ in 0..count {
            self.compile(Instruction::Drop);
        }
    }

    /// Creates a new [`Block`] with the current [`Block`]'s [`Exit`] and
    /// returns its [`Label`].
    fn compile_branch_target(&mut self) -> Label {
        let branch_label = self.cfg.insert_block();
        self.cfg.block_mut(branch_label).exit = self.cfg.block(self.label).exit.clone();
        branch_label
    }

    /// Returns a mutable reference to the current [`Block`].
    fn block_mut(&mut self) -> &mut Block {
        self.cfg.block_mut(self.label)
    }
}
