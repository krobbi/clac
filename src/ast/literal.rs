/// A value that can be represented by a literal token.
#[derive(Debug, Clone)]
pub enum Literal {
    /// A number literal.
    Number(f64),
}
