mod stack;
mod upvars;

use std::mem;

use crate::{
    ast::{BinOp, Literal, UnOp},
    cfg::{Block, Cfg, Exit, Function, Instruction, Label},
    symbols::Symbol,
};

use super::{
    ir::{Expr, Ir, Stmt},
    locals::{Local, LocalTable},
};

use self::{stack::StackFrame, upvars::UpvarStack};

/// Compiles [`Ir`] to a [`Cfg`] with a [`LocalTable`].
pub fn compile_ir(ir: &Ir, locals: &LocalTable) -> Cfg {
    let mut compiler = Compiler::new(locals);
    compiler.compile_ir(ir);
    compiler.into_cfg()
}

/// A structure which compiles a [`Cfg`] from [`Ir`].
struct Compiler<'loc> {
    /// The [`LocalTable`].
    locals: &'loc LocalTable,

    /// The [`UpvarStack`].
    upvars: UpvarStack,

    /// The current [`FunctionContext`].
    function: FunctionContext,

    /// The current function depth.
    function_depth: usize,
}

impl<'loc> Compiler<'loc> {
    /// Creates a new `Compiler` from a [`LocalTable`].
    fn new(locals: &'loc LocalTable) -> Self {
        Self {
            locals,
            upvars: UpvarStack::new(),
            function: FunctionContext::new(0),
            function_depth: 0,
        }
    }

    /// Consumes the `Compiler` and converts it to a [`Cfg`].
    fn into_cfg(self) -> Cfg {
        self.function.cfg
    }

    /// Compiles [`Ir`].
    fn compile_ir(&mut self, ir: &Ir) {
        self.compile_stmts(&ir.0);
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
            Stmt::Block(stmts) => self.compile_stmt_block(stmts),
            Stmt::AssignGlobal(symbol, value) => self.compile_stmt_assign_global(*symbol, value),
            Stmt::DefineLocal(id, value) => self.compile_stmt_define_local(*id, value),
            Stmt::Print(value) => self.compile_stmt_print(value),
            Stmt::Expr(expr) => self.compile_stmt_expr(expr),
        }
    }

    /// Compiles a block [`Stmt`].
    fn compile_stmt_block(&mut self, stmts: &[Stmt]) {
        self.upvars.push_scope();

        self.function.stack_frame.push_scope();
        self.compile_stmts(stmts);
        let local_count = self.function.stack_frame.pop_scope();
        self.compile_drop(local_count);

        let upvar_count = self.upvars.pop_scope();
        self.compile_pop_upvars(upvar_count);
    }

    /// Compiles a global variable assignment [`Stmt`].
    fn compile_stmt_assign_global(&mut self, symbol: Symbol, value: &Expr) {
        self.compile_expr(value);
        self.compile(Instruction::StoreGlobal(symbol));
    }

    /// Compiles a local variable definition [`Stmt`].
    fn compile_stmt_define_local(&mut self, local: Local, value: &Expr) {
        self.compile_expr(value);

        if self.locals.data(local).is_upvar {
            self.compile(Instruction::DefineUpvalue);
            self.upvars.push_upvar(local);
        } else {
            self.function.stack_frame.push_local(local);
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
            Expr::Local(local) => self.compile_expr_local(*local),
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
        self.compile(Instruction::LoadGlobal(symbol));
    }

    /// Compiles a local variable [`Expr`].
    fn compile_expr_local(&mut self, local: Local) {
        let local_data = self.locals.data(local);

        if local_data.is_upvar {
            let offset = self.upvars.upvar_offset(local);
            self.compile(Instruction::LoadUpvalue(offset));
            self.function.access_upvar(local_data.function_depth);
        } else {
            let offset = self.function.stack_frame.local_offset(local);
            self.compile(Instruction::LoadLocal(offset));
        }
    }

    /// Compiles a block [`Expr`].
    fn compile_expr_block(&mut self, stmts: &[Stmt], expr: &Expr) {
        self.upvars.push_scope();

        self.function.stack_frame.push_scope();
        self.compile_stmts(stmts);
        self.compile_expr(expr);
        let local_count = self.function.stack_frame.pop_scope();

        if local_count > 0 {
            // The result of the block expression is on top of the stack, but
            // there are local variables below it which need to be popped. Move
            // the result into the first local variable and pop any local
            // variables above it.
            let offset = self.function.stack_frame.len();
            self.compile(Instruction::StoreLocal(offset));
            self.compile_drop(local_count - 1);
        }

        let upvar_count = self.upvars.pop_scope();
        self.compile_pop_upvars(upvar_count);
    }

    /// Compiles a function [`Expr`].
    fn compile_expr_function(&mut self, params: &[Local], body: &Expr) {
        self.function_depth += 1;
        let mut other_function = mem::replace(
            &mut self.function,
            FunctionContext::new(self.function_depth),
        );

        self.upvars.push_scope();

        // A function's parameters are already on the stack when it is called,
        // but they need to be declared to the compiler.
        for local in params.iter().copied() {
            if self.locals.data(local).is_upvar {
                let offset = self.function.stack_frame.len();
                self.function.stack_frame.push_temp();

                // Upvar parameters are copied to the top of the stack before
                // being defined as upvars. The parameters are already on the
                // stack, so there may be parameters above it which would block
                // this operation.
                self.compile(Instruction::LoadLocal(offset));
                self.compile(Instruction::DefineUpvalue);
                self.upvars.push_upvar(local);
            } else {
                self.function.stack_frame.push_param(local);
            }
        }

        self.compile_expr(body);
        let upvar_count = self.upvars.pop_scope();
        self.compile_pop_upvars(upvar_count);
        self.block_mut().exit = Exit::Return;

        mem::swap(&mut self.function, &mut other_function);
        self.function_depth -= 1;

        let upvar_function_depth = other_function.min_upvar_function_depth;

        self.compile(Instruction::PushFunction(
            Function {
                cfg: other_function.cfg,
                arity: params.len(),
            }
            .into(),
        ));

        if upvar_function_depth <= self.function_depth {
            // The inner function accesses an upvar which is declared outside of
            // it, so it may need to be a closure.
            self.compile(Instruction::IntoClosure);

            // If the accessed upvar is declared outside of the outer function,
            // then the outer function may also need to be a closure.
            self.function.access_upvar(upvar_function_depth);
        }
    }

    /// Compiles a function call [`Expr`].
    fn compile_expr_call(&mut self, callee: &Expr, args: &[Expr]) {
        self.compile_expr(callee);
        self.function.stack_frame.push_temp();

        for arg in args {
            self.compile_expr(arg);
            self.function.stack_frame.push_temp();
        }

        let arity = args.len();
        let return_label = self.cfg_mut().insert_block();
        let terminator = mem::replace(&mut self.block_mut().exit, Exit::Call(arity, return_label));

        self.set_label(return_label);
        self.function.stack_frame.pop_temps(arity + 1);
        self.block_mut().exit = terminator;
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
        self.function.stack_frame.push_temp();
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
        self.function.stack_frame.pop_temps(1);
    }

    /// Compiles a ternary conditional [`Expr`].
    fn compile_expr_cond(&mut self, cond: &Expr, then_expr: &Expr, else_expr: &Expr) {
        self.compile_expr(cond);
        let then_label = self.cfg_mut().insert_block();
        let else_label = self.cfg_mut().insert_block();
        let join_label = self.cfg_mut().insert_block();
        let terminator = mem::replace(
            &mut self.block_mut().exit,
            Exit::Branch(then_label, else_label),
        );

        self.set_label(then_label);
        self.compile_expr(then_expr);
        self.block_mut().exit = Exit::Jump(join_label);

        self.set_label(else_label);
        self.compile_expr(else_expr);
        self.block_mut().exit = Exit::Jump(join_label);

        self.set_label(join_label);
        self.block_mut().exit = terminator;
    }

    /// Returns a mutable reference to the current [`Cfg`].
    const fn cfg_mut(&mut self) -> &mut Cfg {
        &mut self.function.cfg
    }

    /// Returns a mutable reference to the current [`Block`].
    fn block_mut(&mut self) -> &mut Block {
        let label = self.function.label;
        self.cfg_mut().block_mut(label)
    }

    /// Sets the current [`Label`].
    const fn set_label(&mut self, label: Label) {
        self.function.label = label;
    }

    /// Appends [`Instruction`]s to drop multiple values to the current
    /// [`Block`].
    fn compile_drop(&mut self, count: usize) {
        if count > 0 {
            self.compile(Instruction::Drop(count));
        }
    }

    /// Appends [`Instruction`]s to pop multiple upvalues to the current
    /// [`Block`].
    fn compile_pop_upvars(&mut self, count: usize) {
        if count > 0 {
            self.compile(Instruction::DropUpvalues(count));
        }
    }

    /// Appends an [`Instruction`] to the current [`Block`].
    fn compile(&mut self, instruction: Instruction) {
        self.block_mut().instructions.push(instruction);
    }
}

/// Context for compiling a [`Function`].
struct FunctionContext {
    /// The [`Cfg`].
    cfg: Cfg,

    /// The current [`Label`].
    label: Label,

    /// The [`StackFrame`].
    stack_frame: StackFrame,

    /// The minimum function depth where an accessed upvar was declared.
    min_upvar_function_depth: usize,
}

impl FunctionContext {
    /// Creates a new `FunctionContext` at a function depth.
    fn new(function_depth: usize) -> Self {
        Self {
            cfg: Cfg::new(),
            label: Label::default(),
            stack_frame: StackFrame::new(),
            min_upvar_function_depth: function_depth,
        }
    }

    /// Marks an upvar being accessed at a function depth.
    fn access_upvar(&mut self, function_depth: usize) {
        self.min_upvar_function_depth = self.min_upvar_function_depth.min(function_depth);
    }
}
