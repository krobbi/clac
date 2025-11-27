use crate::{cfg::UpvalueId, decl_table::DeclId};

/// A lookup table of [`UpvalueId`]s.
#[derive(Default)]
pub struct UpvalueTable {
    /// The [`DeclId`]s that are declared as upvalues.
    decl_ids: Vec<DeclId>,
}

impl UpvalueTable {
    /// Creates a new `UpvalueTable`
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares a [`DeclId`] as an upvalue and returns its [`UpvalueId`].
    pub fn declare(&mut self, id: DeclId) -> UpvalueId {
        let index = self.decl_ids.len();
        self.decl_ids.push(id);
        UpvalueId(index)
    }
}
