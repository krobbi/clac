use std::{
    cell::RefCell,
    fmt::{self, Display, Formatter},
};

// NOTE: Symbols will break if they are not all created and displayed in the
// same thread.
thread_local! {
    // HACK: Storing symbol names globally allows symbols to be displayed
    // without a reference to a symbol table. This allows symbols to be used
    // directly in error messages.
    /// The interned names.
    static NAMES: RefCell<Vec<Box<str>>> = const { RefCell::new(Vec::new()) };
}

/// An interned name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(usize);

impl Symbol {
    /// Interns a name and returns its `Symbol`.
    pub fn intern(name: &str) -> Self {
        let index = NAMES.with_borrow_mut(|v| {
            v.iter()
                .position(|n| n.as_ref() == name)
                .unwrap_or_else(|| {
                    v.push(name.into());
                    v.len() - 1
                })
        });

        Self(index)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        NAMES.with_borrow(|v| f.write_str(&v[self.0]))
    }
}
