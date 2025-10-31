use crate::{
    ast::{Ast, Expr, Stmt},
    ir::{Body, Instruction, Ir, Value},
};

/// Compiles an [`Ast`] to [`Ir`].
pub fn compile_ast(ast: &Ast) -> Ir {
    let mut compiler = Compiler::new();
    compiler.compile_ast(ast);
    Ir(compiler.into_body())
}

/// A structure that compiles a program or function's [`Body`].
struct Compiler {
    /// The [`Instruction`]s that have been compiled.
    instructions: Vec<Instruction>,
}

impl Compiler {
    /// Creates a new `Compiler`.
    fn new() -> Self {
        let instructions = Vec::new();
        Self { instructions }
    }

    /// Consumes the `Compiler` and converts it to a [`Body`].
    fn into_body(self) -> Body {
        Body(self.instructions.into_boxed_slice())
    }

    /// Compiles an [`Ast`].
    fn compile_ast(&mut self, ast: &Ast) {
        for stmt in &ast.0 {
            match stmt {
                Stmt::Assign(..) => todo!("compilation of `Stmt::Assign`"),
                Stmt::Expr(expr) => {
                    self.compile_expr(expr);
                    self.compile(Instruction::Print);
                }
            }
        }

        self.compile(Instruction::Halt);
    }

    /// Compiles an [`Expr`].
    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(value) => self.compile(Instruction::PushValue(Value::Number(*value))),
            Expr::Ident(..) => todo!("compilation of `Expr::Ident`"),
            Expr::Paren(expr) => self.compile_expr(expr),
            Expr::Block(..) => todo!("compilation of `Expr::Block`"),
            Expr::Call(..) => todo!("compilation of `Expr::Call`"),
            Expr::Unary(..) => todo!("compilation of `Expr::Unary`"),
            Expr::Binary(..) => todo!("compilation of `Expr::Binary`"),
        }
    }

    /// Appends an [`Instruction`] to the current block.
    fn compile(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}
