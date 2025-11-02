mod compile_error;
mod locals;

pub use self::compile_error::CompileError;

use std::collections::HashSet;

use crate::{
    ast::{Ast, BinOp, Expr, Stmt, UnOp},
    ir::{self, Body, Instruction, Ir, Value},
};

use self::locals::Locals;

/// Compiles an [`Ast`] to [`Ir`] with a [`HashSet`] of defined global variable
/// names. This function returns a [`CompileError`] if [`Ir`] could not be
/// compiled.
pub fn compile_ast(ast: &Ast, globals: HashSet<String>) -> Result<Ir, CompileError> {
    let mut compiler = Compiler::new(globals);
    compiler.compile_ast(ast)?;
    Ok(Ir(compiler.into_body()))
}

/// A structure that compiles a program or function's [`Body`].
struct Compiler {
    /// The set of declared global variables.
    globals: HashSet<String>,

    /// The stack of declared local variables.
    locals: Locals,

    /// The [`Instruction`]s that have been compiled.
    instructions: Vec<Instruction>,
}

impl Compiler {
    /// Creates a new `Compiler` with a [`HashSet`] of defined global variable
    /// names.
    fn new(globals: HashSet<String>) -> Self {
        let locals = Locals::new();
        let instructions = Vec::new();

        Self {
            globals,
            locals,
            instructions,
        }
    }

    /// Consumes the `Compiler` and converts it to a [`Body`].
    fn into_body(self) -> Body {
        Body(self.instructions.into_boxed_slice())
    }

    /// Compiles an [`Ast`]. This function returns a [`CompileError`] if the
    /// [`Ast`] could not be compiled.
    fn compile_ast(&mut self, ast: &Ast) -> Result<(), CompileError> {
        for stmt in &ast.0 {
            match stmt {
                Stmt::Assign(target, source) => self.compile_assign_stmt(target, source)?,
                Stmt::Expr(expr) => {
                    self.compile_expr(expr)?;
                    self.compile(Instruction::Print);
                }
            }
        }

        self.compile(Instruction::Halt);
        Ok(())
    }

    /// Compiles an assignment [`Stmt`]. This function returns a
    /// [`CompileError`] if the assignment source or target was invalid.
    fn compile_assign_stmt(&mut self, target: &Expr, source: &Expr) -> Result<(), CompileError> {
        let Expr::Ident(name) = target else {
            return Err(CompileError::InvalidAssignTarget);
        };

        self.compile_expr(source)?;

        if self.locals.is_global_scope() {
            if self.globals.contains(name) {
                return Err(CompileError::AlreadyDefinedVariable(name.to_owned()));
            }

            self.compile(Instruction::StoreGlobal(name.to_owned()));
            self.globals.insert(name.to_owned());
        } else if !self.locals.declare(name) {
            return Err(CompileError::AlreadyDefinedVariable(name.to_owned()));
        }

        Ok(())
    }

    /// Compiles an [`Expr`]. This function returns a [`CompileError`] if the
    /// [`Expr`] could not be compiled.
    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Number(value) => {
                self.compile(Instruction::Push(Value::Number(*value)));
                Ok(())
            }
            Expr::Ident(name) => self.compile_expr_ident(name),
            Expr::Paren(expr) => self.compile_expr(expr),
            Expr::Block(stmts) => self.compile_expr_block(stmts),
            Expr::Call(..) => todo!("compilation of `Expr::Call`"),
            Expr::Unary(op, expr) => self.compile_expr_unary(*op, expr),
            Expr::Binary(op, lhs, rhs) => self.compile_expr_binary(*op, lhs, rhs),
        }
    }

    /// Compiles an identifier [`Expr`]. This function returns a
    /// [`CompileError`] if no variable is defined with the identifier's name.
    fn compile_expr_ident(&mut self, name: &str) -> Result<(), CompileError> {
        if let Some(index) = self.locals.get(name) {
            self.compile(Instruction::PushLocal(index));
        } else if self.globals.contains(name) {
            self.compile(Instruction::PushGlobal(name.to_owned()));
        } else {
            return Err(CompileError::UndefinedVariable(name.to_owned()));
        }

        Ok(())
    }

    /// Compiles a block [`Expr`]. This function returns a [`CompileError`] if
    /// any of the block's [`Stmt`]s could not be compiled.
    fn compile_expr_block(&mut self, stmts: &[Stmt]) -> Result<(), CompileError> {
        let mut stmts = stmts.iter();
        let mut is_void = true;
        self.locals.push_scope();

        while let Some(stmt) = stmts.next() {
            match stmt {
                Stmt::Assign(target, source) => self.compile_assign_stmt(target, source)?,
                Stmt::Expr(expr) => {
                    self.compile_expr(expr)?;

                    if stmts.len() > 0 {
                        self.compile(Instruction::Pop);
                    } else {
                        is_void = false;
                    }
                }
            }
        }

        let block_variable_count = self.locals.pop_scope();

        if block_variable_count > 0 {
            if is_void {
                self.compile_pop(block_variable_count);
            } else {
                // HACK: If the block declared local variables and produced a
                // result, then move the result into the first local variable
                // and don't pop it.
                self.compile(Instruction::StoreLocal(self.locals.count()));
                self.compile_pop(block_variable_count - 1);
            }
        }

        if is_void {
            // Push void at the end to bypass the stack manipulation hack.
            self.compile(Instruction::Push(Value::Void));
        }

        Ok(())
    }

    /// Compiles a unary [`Expr`]. This function returns a [`CompileError`] if
    /// the operand could not be compiled.
    fn compile_expr_unary(&mut self, op: UnOp, expr: &Expr) -> Result<(), CompileError> {
        self.compile_expr(expr)?;

        let op = match op {
            UnOp::Negate => ir::UnOp::Negate,
        };

        self.compile(Instruction::Unary(op));
        Ok(())
    }

    /// Compiles a binary [`Expr`]. This function returns a [`CompileError`] if
    /// either operand could not be compiled.
    fn compile_expr_binary(
        &mut self,
        op: BinOp,
        lhs: &Expr,
        rhs: &Expr,
    ) -> Result<(), CompileError> {
        self.compile_expr(lhs)?;

        // HACK: `rhs` could be a block that defines local variables, but the
        // location of these variables would be off by one because `lhs` is
        // sitting on top the stack. To fix this, `lhs` is treated as an unnamed
        // local variable while `rhs` is being evaluated.
        self.locals.push_temp();
        self.compile_expr(rhs)?;
        self.locals.pop_temp();

        let op = match op {
            BinOp::Add => ir::BinOp::Add,
            BinOp::Subtract => ir::BinOp::Subtract,
            BinOp::Multiply => ir::BinOp::Multiply,
            BinOp::Divide => ir::BinOp::Divide,
        };

        self.compile(Instruction::Binary(op));
        Ok(())
    }

    /// Appends an [`Instruction`] to the current block.
    fn compile(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    /// Appends `n` pop [`Instruction`]s to the current block.
    fn compile_pop(&mut self, n: usize) {
        for _ in 0..n {
            self.compile(Instruction::Pop);
        }
    }
}
