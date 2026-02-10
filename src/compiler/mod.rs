mod local_stack;
mod upvalue_stack;

use std::mem;

use crate::{
    ast::{BinOp, Literal, UnOp},
    cfg::{Block, Cfg, Exit, Function, Instruction, Label},
    decl_table::{DeclId, DeclTable},
    hir::{Expr, Hir, Stmt},
    symbols::Symbol,
};

use self::{local_stack::LocalStack, upvalue_stack::UpvalueStack};

/// Compiles [`Hir`] to a [`Cfg`] with a [`DeclTable`].
pub fn compile_hir(hir: &Hir, decls: &DeclTable) -> Cfg {
    let mut compiler = Compiler::new(decls);
    compiler.compile_hir(hir);
    compiler.into_cfg()
}

/// A structure that compiles [`Hir`] to a [`Cfg`].
struct Compiler<'a> {
    /// The [`DeclTable`].
    decls: &'a DeclTable,

    /// The current call depth.
    call_depth: usize,

    /// The [`LocalStack`].
    locals: LocalStack,

    /// The [`UpvalueStack`].
    upvalues: UpvalueStack,

    /// The current [`Label`].
    label: Label,

    /// The [`Cfg`].
    cfg: Cfg,
}

impl<'a> Compiler<'a> {
    /// Creates a new `Compiler` from a [`DeclTable`].
    fn new(decls: &'a DeclTable) -> Self {
        Self {
            decls,
            call_depth: 0,
            locals: LocalStack::new(0),
            upvalues: UpvalueStack::new(),
            label: Label::default(),
            cfg: Cfg::new(),
        }
    }

    /// Consumes the `Compiler` and converts it to a [`Cfg`].
    fn into_cfg(self) -> Cfg {
        self.cfg
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
            Stmt::AssignGlobal(symbol, value) => self.compile_stmt_assign_global(*symbol, value),
            Stmt::DefineLocal(id, value) => self.compile_stmt_define_local(*id, value),
            Stmt::Print(value) => self.compile_stmt_print(value),
            Stmt::Expr(expr) => self.compile_stmt_expr(expr),
        }
    }

    /// Compiles a block [`Stmt`].
    fn compile_stmt_block(&mut self, stmts: &[Stmt]) {
        self.upvalues.begin_scope();

        self.locals.begin_block();
        self.compile_stmts(stmts);
        let local_count = self.locals.end_block();
        self.compile_drop(local_count);

        let upvalue_count = self.upvalues.end_scope();
        self.compile_drop_upvalues(upvalue_count);
    }

    /// Compiles a global variable assignment [`Stmt`].
    fn compile_stmt_assign_global(&mut self, symbol: Symbol, value: &Expr) {
        self.compile_expr(value);
        self.compile(Instruction::StoreGlobal(symbol.to_string()));
    }

    /// Compiles a local variable definition [`Stmt`].
    fn compile_stmt_define_local(&mut self, id: DeclId, value: &Expr) {
        self.compile_expr(value);

        if self.decls.get(id).is_upvalue {
            self.compile(Instruction::DefineUpvalue);
            self.upvalues.declare(id);
        } else {
            self.locals.declare(id);
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
        self.compile(Instruction::Drop(1));
    }

    /// Compiles an [`Expr`].
    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(literal) => self.compile_expr_literal(literal),
            Expr::Global(symbol) => self.compile_expr_global(*symbol),
            Expr::Local(id) => self.compile_expr_local(*id),
            Expr::Block(stmts, expr) => self.compile_expr_block(stmts, expr),
            Expr::Function(params, body) => self.compile_expr_function(params, body),
            Expr::Call(callee, args) => self.compile_expr_call(callee, args),
            Expr::Unary(op, rhs) => self.compile_expr_unary(*op, rhs),
            Expr::Binary(op, lhs, rhs) => self.compile_expr_binary(*op, lhs, rhs),
            Expr::Cond(cond, then, or) => self.compile_expr_cond(cond, then, or),
        }
    }

    /// Compiles a literal [`Expr`].
    fn compile_expr_literal(&mut self, literal: &Literal) {
        self.compile(Instruction::PushLiteral(literal.clone()));
    }

    /// Compiles a global variable [`Expr`].
    fn compile_expr_global(&mut self, symbol: Symbol) {
        self.compile(Instruction::LoadGlobal(symbol.to_string()));
    }

    /// Compiles a local variable [`Expr`].
    fn compile_expr_local(&mut self, id: DeclId) {
        let decl = self.decls.get(id);

        if decl.is_upvalue {
            let offset = self.upvalues.offset(id);
            self.compile(Instruction::LoadUpvalue(offset));
            self.locals.access_upvalue(decl.call_depth);
        } else {
            let offset = self.locals.offset(id);
            self.compile(Instruction::LoadLocal(offset));
        }
    }

    /// Compiles a block [`Expr`].
    fn compile_expr_block(&mut self, stmts: &[Stmt], expr: &Expr) {
        self.upvalues.begin_scope();

        self.locals.begin_block();
        self.compile_stmts(stmts);
        self.compile_expr(expr);
        let local_count = self.locals.end_block();

        if local_count > 0 {
            // The result of the block expression is on top of the stack, but
            // there are local variables below it that need to be dropped. Move
            // the result into the first local variable and drop any local
            // variables above it.
            let offset = self.locals.len();
            self.compile(Instruction::StoreLocal(offset));
            self.compile_drop(local_count - 1);
        }

        let upvalue_count = self.upvalues.end_scope();
        self.compile_drop_upvalues(upvalue_count);
    }

    /// Compiles a function [`Expr`].
    fn compile_expr_function(&mut self, params: &[DeclId], body: &Expr) {
        self.call_depth += 1;
        let outer_locals = mem::replace(&mut self.locals, LocalStack::new(self.call_depth));

        let outer_label = self.label;
        let outer_cfg = mem::replace(&mut self.cfg, Cfg::new());
        self.label = Label::default();

        self.upvalues.begin_scope();

        // The function's arguments are already on the stack, but need to be
        // declared.
        for id in params.iter().copied() {
            if self.decls.get(id).is_upvalue {
                let offset = self.locals.len();
                self.locals.push_temp();

                // Upvalue arguments are copied from the stack before they are
                // defined as upvalues. The caller has already placed all of the
                // arguments on the stack, so the top of the stack may not be
                // the upvalue that is expected.
                self.compile(Instruction::LoadLocal(offset));
                self.compile(Instruction::DefineUpvalue);
                self.upvalues.declare(id);
            } else {
                self.locals.declare(id);
            }
        }

        self.compile_expr(body);

        let upvalue_count = self.upvalues.end_scope();
        self.compile_drop_upvalues(upvalue_count);

        self.block_mut().exit = Exit::Return;

        self.label = outer_label;
        let function_cfg = mem::replace(&mut self.cfg, outer_cfg);

        let upvalue_call_depth = self.locals.upvalue_call_depth();
        self.locals = outer_locals;
        self.call_depth -= 1;

        self.compile(Instruction::PushFunction(
            Function {
                cfg: function_cfg,
                arity: params.len(),
            }
            .into(),
        ));

        if upvalue_call_depth <= self.call_depth {
            // The outer function could outlive an upvalue accessed by the inner
            // function, so it may need to be a closure.
            self.locals.access_upvalue(upvalue_call_depth);

            // The inner function could outlive an upvalue that it accesses, so
            // it must be converted to a closure.
            self.compile(Instruction::IntoClosure);
        }
    }

    /// Compiles a function call [`Expr`].
    fn compile_expr_call(&mut self, callee: &Expr, args: &[Expr]) {
        self.compile_expr(callee);
        self.locals.push_temp();

        for arg in args {
            self.compile_expr(arg);
            self.locals.push_temp();
        }

        let arity = args.len();
        let return_label = self.compile_split_block();
        self.block_mut().exit = Exit::Call(arity, return_label);
        self.label = return_label;
        self.locals.drop_temps(arity + 1);
    }

    /// Compiles a unary [`Expr`].
    fn compile_expr_unary(&mut self, op: UnOp, rhs: &Expr) {
        self.compile_expr(rhs);

        let instruction = match op {
            UnOp::Negate => Instruction::Negate,
            UnOp::Not => Instruction::Not,
        };

        self.compile(instruction);
    }

    /// Compiles a binary [`Expr`].
    fn compile_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) {
        self.compile_expr(lhs);
        self.locals.push_temp();
        self.compile_expr(rhs);

        let instruction = match op {
            BinOp::Add => Instruction::Add,
            BinOp::Subtract => Instruction::Subtract,
            BinOp::Multiply => Instruction::Multiply,
            BinOp::Divide => Instruction::Divide,
            BinOp::Power => Instruction::Power,
            BinOp::Equal => Instruction::Equal,
            BinOp::NotEqual => Instruction::NotEqual,
            BinOp::Less => Instruction::Less,
            BinOp::LessEqual => Instruction::LessEqual,
            BinOp::Greater => Instruction::Greater,
            BinOp::GreaterEqual => Instruction::GreaterEqual,
        };

        self.compile(instruction);
        self.locals.drop_temps(1);
    }

    /// Compiles a ternary conditional [`Expr`].
    fn compile_expr_cond(&mut self, cond: &Expr, then: &Expr, or: &Expr) {
        self.compile_expr(cond);
        let final_label = self.compile_split_block();
        let then_label = self.compile_source_block(final_label);
        let else_label = self.compile_source_block(final_label);
        self.block_mut().exit = Exit::Branch(then_label, else_label);

        self.label = then_label;
        self.compile_expr(then);

        self.label = else_label;
        self.compile_expr(or);

        self.label = final_label;
    }

    /// Appends an [`Instruction`] to the current [`Block`].
    fn compile(&mut self, instruction: Instruction) {
        self.block_mut().instructions.push(instruction);
    }

    /// Appends [`Instruction`]s to drop multiple values to the current
    /// [`Block`].
    fn compile_drop(&mut self, count: usize) {
        if count > 0 {
            self.compile(Instruction::Drop(count));
        }
    }

    /// Appends [`Instruction`]s to drop multiple upvalues to the current
    /// [`Block`].
    fn compile_drop_upvalues(&mut self, count: usize) {
        if count > 0 {
            self.compile(Instruction::DropUpvalues(count));
        }
    }

    /// Creates a new [`Block`] that jumps to a target [`Label`] and returns its
    /// [`Label`].
    fn compile_source_block(&mut self, target: Label) -> Label {
        let source_label = self.cfg.insert_block();
        self.cfg.block_mut(source_label).exit = Exit::Jump(target);
        source_label
    }

    /// Creates a new [`Block`] with the current [`Block`]'s [`Exit`] and
    /// returns its [`Label`].
    fn compile_split_block(&mut self) -> Label {
        let branch_label = self.cfg.insert_block();
        self.cfg.block_mut(branch_label).exit = self.cfg.block(self.label).exit.clone();
        branch_label
    }

    /// Returns a mutable reference to the current [`Block`].
    fn block_mut(&mut self) -> &mut Block {
        self.cfg.block_mut(self.label)
    }
}
