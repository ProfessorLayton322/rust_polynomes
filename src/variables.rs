#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Var(pub usize);

pub const X : Var = Var(0usize);

pub const Y : Var = Var(1usize);

pub const Z : Var = Var(2usize);
