/// A unique identifier for a local variable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Local(usize);

/// A table of [`Local`]s.
#[derive(Default)]
pub struct LocalTable {
    /// The [`Data`].
    data: Vec<Data>,
}

impl LocalTable {
    /// Creates a new `LocalTable`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to a [`Local`]'s data.
    pub fn data(&self, local: Local) -> &Data {
        &self.data[local.0]
    }

    /// Returns a mutable reference to a [`Local`]'s data.
    pub fn data_mut(&mut self, local: Local) -> &mut Data {
        &mut self.data[local.0]
    }

    /// Declares a new [`Local`] at a function depth.
    pub fn declare_local(&mut self, function_depth: usize) -> Local {
        self.data.push(Data {
            function_depth,
            is_upvar: false,
        });

        Local(self.data.len() - 1)
    }
}

/// A [`Local`]'s data.
pub struct Data {
    /// The function depth where the [`Local`] is declared.
    pub function_depth: usize,

    /// Whether the [`Local`] is an upvar.
    pub is_upvar: bool,
}
