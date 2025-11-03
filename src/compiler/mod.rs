mod stack;

use crate::{
    hir::{BinOp, Expr, Hir, Stmt},
    ir::{self, Body, Function, Instruction, Ir, Value},
};

use self::stack::Stack;

/// Compiles [`Hir`] to [`Ir`].
pub fn compile_hir(hir: &Hir) -> Ir {
    let mut compiler = Compiler::new();
    compiler.compile_hir(hir);
    Ir(compiler.into_body())
}

/// A structure that compiles a program or function's [`Body`].
struct Compiler {
    /// The [`Stack`] for tracking the locations of local variables.
    stack: Stack,

    /// The [`Instruction`]s that have been compiled.
    instructions: Vec<Instruction>,
}

impl Compiler {
    /// Creates a new `Compiler`.
    fn new() -> Self {
        let stack = Stack::new();
        let instructions = Vec::new();

        Self {
            stack,
            instructions,
        }
    }

    /// Consumes the `Compiler` and converts it to a [`Body`].
    fn into_body(self) -> Body {
        Body(self.instructions.into_boxed_slice())
    }

    /// Compiles [`Hir`].
    fn compile_hir(&mut self, hir: &Hir) {
        self.compile_stmts(&hir.0);
        self.compile(Instruction::Halt);
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
            Stmt::DefineLocal(name, value) => self.compile_stmt_define_local(name, value),
            Stmt::AssignGlobal(name, value) => self.compile_stmt_assign_global(name, value),
            Stmt::Print(value) => self.compile_stmt_print(value),
            Stmt::Expr(expr) => self.compile_stmt_expr(expr),
        }
    }

    /// Compiles a block [`Stmt`].
    fn compile_stmt_block(&mut self, stmts: &[Stmt]) {
        self.stack.push_scope();
        self.compile_stmts(stmts);
        let local_count = self.stack.pop_scope();
        self.compile_drop(local_count);
    }

    /// Compiles a local variable definition [`Stmt`].
    fn compile_stmt_define_local(&mut self, name: &str, value: &Expr) {
        self.compile_expr(value);
        self.stack.declare_local(name);
    }

    /// Compiles a global variable assignment [`Stmt`].
    fn compile_stmt_assign_global(&mut self, name: &str, value: &Expr) {
        self.compile_expr(value);
        self.compile(Instruction::StoreGlobal(name.to_owned()));
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
            Expr::Number(value) => self.compile(Instruction::Push(Value::Number(*value))),
            Expr::Local(name) => self.compile(Instruction::LoadLocal(self.stack.local_index(name))),
            Expr::Global(name) => self.compile(Instruction::LoadGlobal(name.to_owned())),
            Expr::Function(params, body) => self.compile_expr_function(params, body),
            Expr::Block(stmts, expr) => self.compile_expr_block(stmts, expr),
            Expr::Binary(op, lhs, rhs) => self.compile_expr_binary(*op, lhs, rhs),
        }
    }

    /// Compiles a function [`Expr`].
    fn compile_expr_function(&mut self, params: &[String], body: &Expr) {
        let mut compiler = Self::new();

        for param in params {
            compiler.stack.declare_local(param);
        }

        compiler.compile_expr(body);
        compiler.compile(Instruction::Return);
        let value = Value::Function(Function(params.len(), compiler.into_body()).into());
        self.compile(Instruction::Push(value));
    }

    /// Compiles a block [`Expr`].
    fn compile_expr_block(&mut self, stmts: &[Stmt], expr: &Expr) {
        self.stack.push_scope();
        self.compile_stmts(stmts);
        self.compile_expr(expr);
        let local_count = self.stack.pop_scope();

        if local_count > 0 {
            // The result of the block expression is on top of the stack, but
            // there are local variables below it that need to be dropped. Move
            // the result into the first local variable and drop any local
            // variables above it.
            self.compile(Instruction::StoreLocal(self.stack.len()));
            self.compile_drop(local_count - 1);
        }
    }

    /// Compiles a binary [`Expr`].
    fn compile_expr_binary(&mut self, op: BinOp, lhs: &Expr, rhs: &Expr) {
        self.compile_expr(lhs);
        self.stack.declare_intermediate();
        self.compile_expr(rhs);

        let op = match op {
            BinOp::Add => ir::BinOp::Add,
            BinOp::Subtract => ir::BinOp::Subtract,
            BinOp::Multiply => ir::BinOp::Multiply,
            BinOp::Divide => ir::BinOp::Divide,
        };

        self.compile(Instruction::Binary(op));
        self.stack.declare_drop_intermediate();
    }

    /// Appends an [`Instruction`] to the current block.
    fn compile(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Appends multiple drop [`Instruction`]s to the current block.
    fn compile_drop(&mut self, count: usize) {
        for _ in 0..count {
            self.compile(Instruction::Drop);
        }
    }
}
