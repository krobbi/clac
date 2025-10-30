use crate::{ast::Ast, hir::Hir};

/// Validates an [`Ast`] and lowers it to [`Hir`].
pub fn resolve_ast(_ast: &Ast) -> Hir {
    Hir
}
