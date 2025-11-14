mod stack;

use crate::{
    decl_table::{DeclId, DeclTable},
    hir::{BinOp, Expr, Hir, Stmt},
    ir::{self, Body, Function, Instruction, Ir, Value},
};

use self::stack::Stack;

/// Compiles [`Hir`] to [`Ir`] with a [`DeclTable`].
pub fn compile_hir(hir: &Hir, decls: &DeclTable) -> Ir {
    let mut compiler = Compiler::new(decls);
    compiler.compile_hir(hir);
    Ir(compiler.into_body())
}

/// A structure that compiles a program or function's [`Body`].
struct Compiler<'a> {
    /// The [`DeclTable`].
    decls: &'a DeclTable,

    /// The [`Stack`] for tracking the locations of local variables.
    stack: Stack,

    /// The [`Instruction`]s that have been compiled.
    instructions: Vec<Instruction>,
}

impl<'a> Compiler<'a> {
    /// Creates a new `Compiler`.
    fn new(decls: &'a DeclTable) -> Self {
        Self {
            decls,
            stack: Stack::new(),
            instructions: Vec::new(),
        }
    }

    /// Consumes the `Compiler` and converts it to a [`Body`].
    fn into_body(self) -> Body {
        Body(self.instructions.into_boxed_slice())
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
            Stmt::DeclareLocal(id, value) => self.compile_stmt_declare_local(*id, value),
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

    /// Compiles a global variable assignment [`Stmt`].
    fn compile_stmt_assign_global(&mut self, name: &str, value: &Expr) {
        self.compile_expr(value);
        self.compile(Instruction::StoreGlobal(name.to_owned()));
    }

    /// Compiles a local variable declaration [`Stmt`].
    fn compile_stmt_declare_local(&mut self, id: DeclId, value: &Expr) {
        self.compile_expr(value);

        if self.decls.get(id).is_upvalue {
            self.compile(Instruction::DeclareUpvalue(id));
        } else {
            self.stack.declare_local(id);
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
            Expr::Number(value) => self.compile(Instruction::Push(Value::Number(*value))),
            Expr::Global(name) => self.compile(Instruction::LoadGlobal(name.to_owned())),
            Expr::Local(id) => self.compile_expr_local(*id),
            Expr::Block(stmts, expr) => self.compile_expr_block(stmts, expr),
            Expr::Function(params, body) => self.compile_expr_function(params, body),
            Expr::Call(callee, args) => self.compile_expr_call(callee, args),
            Expr::Binary(op, lhs, rhs) => self.compile_expr_binary(*op, lhs, rhs),
        }
    }

    /// Compiles a local variable [`Expr`].
    fn compile_expr_local(&mut self, id: DeclId) {
        if self.decls.get(id).is_upvalue {
            self.compile(Instruction::LoadUpvalue(id));
        } else {
            self.compile(Instruction::LoadLocal(self.stack.local_offset(id)));
        }
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

    /// Compiles a function [`Expr`].
    fn compile_expr_function(&mut self, params: &[DeclId], body: &Expr) {
        let mut compiler = Self::new(self.decls);

        for id in params {
            let id = *id;

            // All of the arguments to the function are passed on the stack,
            // even if they are unused or used as upvalues. Every argument is
            // declared as a local variable to ensure that other local variables
            // use the correct stack offset.
            compiler.stack.declare_local(id);

            // Upvalue arguments must be copied from the stack to an upvalue.
            // The declaration table will prevent usages of these arguments from
            // using the redundant copy on the stack.
            if self.decls.get(id).is_upvalue {
                compiler.compile(Instruction::LoadLocal(compiler.stack.local_offset(id)));
                compiler.compile(Instruction::DeclareUpvalue(id));
            }
        }

        compiler.compile_expr(body);

        let function = Function {
            arity: params.len(),
            body: compiler.into_body(),
        };

        self.compile(Instruction::Push(Value::Function(function.into())));

        // HACK: Convert all functions to closures. Not all functions need to be
        // closures, but this ensures that functions will behave correctly.
        // Unfortunately, this adds an unnecessary performance and memory cost
        // to most functions.
        self.compile(Instruction::IntoClosure);
    }

    /// Compiles a function call [`Expr`].
    fn compile_expr_call(&mut self, callee: &Expr, args: &[Expr]) {
        self.compile_expr(callee);
        self.stack.declare_intermediate();

        for arg in args {
            self.compile_expr(arg);
            self.stack.declare_intermediate();
        }

        let arity = args.len();
        self.compile(Instruction::Call(arity));

        for _ in 0..=arity {
            self.stack.declare_drop_intermediate();
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
