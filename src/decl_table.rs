/// A lookup table of [`Decl`]s.
#[derive(Default)]
pub struct DeclTable {
    /// The [`Decl`]s.
    decls: Vec<Decl>,
}

impl DeclTable {
    /// Creates a new `DeclTable`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Declares a new [`Decl`] at a call depth and returns its [`DeclId`].
    pub fn declare(&mut self, call_depth: usize) -> DeclId {
        let index = self.decls.len();

        self.decls.push(Decl {
            is_upvalue: false,
            call_depth,
        });

        DeclId(index)
    }

    /// Returns a reference to a [`Decl`] from its [`DeclId`].
    pub fn get(&self, id: DeclId) -> &Decl {
        self.decls.get(id.0).expect("declaration should exist")
    }

    /// Returns a mutable reference to a [`Decl`] from its [`DeclId`].
    pub fn get_mut(&mut self, id: DeclId) -> &mut Decl {
        self.decls.get_mut(id.0).expect("declaration should exist")
    }
}

/// A unique identifier for a [`Decl`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DeclId(usize);

/// A local variable declaration.
pub struct Decl {
    /// Whether the `Decl` is accessed as an upvalue.
    pub is_upvalue: bool,

    /// The call depth the `Decl` is declared at.
    pub call_depth: usize,
}
