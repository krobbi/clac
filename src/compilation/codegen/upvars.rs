use super::super::locals::Local;

/// A stack of upvars.
#[derive(Default)]
pub struct UpvarStack {
    /// The stack of [`Local`]s which are accessible as upvars.
    upvars: Vec<Local>,

    /// The upvar stack offsets to each upvar scope.
    scope_offsets: Vec<usize>,
}

impl UpvarStack {
    /// Creates a new `UpvarStack`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an upvar's upvar stack offset from its [`Local`].
    pub fn upvar_offset(&self, local: Local) -> usize {
        self.upvars
            .iter()
            .position(|l| *l == local)
            .expect("upvar should exist")
    }

    /// Pushes a new scope to the `UpvarStack`.
    pub fn push_scope(&mut self) {
        self.scope_offsets.push(self.upvars.len());
    }

    /// Pops the current scope from the `UpvarStack` and returns the number of
    /// upvars that were declared in the scope.
    pub fn pop_scope(&mut self) -> usize {
        let offset = self
            .scope_offsets
            .pop()
            .expect("there should be an upvar scope");

        let upvar_count = self.upvars.len() - offset;
        self.upvars.truncate(offset);
        upvar_count
    }

    /// Marks an upvar being pushed to the `UpvarStack`.
    pub fn push_upvar(&mut self, local: Local) {
        self.upvars.push(local);
    }
}
