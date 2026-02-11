mod codegen;
mod ir;
mod locals;
mod lowering;

use thiserror::Error;

use crate::{ast::Ast, cfg::Cfg, interpreter::Globals};

use self::{locals::LocalTable, lowering::LoweringError};

/// An error caught while compiling an [`Ast`] to a [`Cfg`].
#[derive(Debug, Error)]
#[repr(transparent)]
#[error(transparent)]
pub struct CompilationError(#[from] LoweringError);

/// Compiles an [`Ast`] to a [`Cfg`] with [`Globals`]. This function returns a
/// [`CompilationError`] if an error occurred.
pub fn compile_ast(ast: &Ast, globals: &Globals) -> Result<Cfg, CompilationError> {
    let mut locals = LocalTable::new();
    let ir = lowering::lower_ast(ast, globals, &mut locals)?;
    Ok(codegen::compile_ir(&ir, &locals))
}
