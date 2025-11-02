/// A kind of scope that a [`Stmt`][crate::hir::Stmt] may appear in.
#[derive(Clone, Copy)]
pub enum ScopeKind {
    /// At the top level of a program.
    Global,

    /// Inside a block.
    Local,
}
