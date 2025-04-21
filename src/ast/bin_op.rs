/// A binary operator.
#[derive(Clone, Copy)]
pub enum BinOp {
    /// An assignment operator.
    Assign,

    /// An addition operator.
    Add,

    /// A subtraction operator.
    Sub,

    /// A multiplication operator.
    Mul,

    /// A division operator.
    Div,
}
