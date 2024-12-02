#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Var {
    pub index: usize,
}

pub const X: Var = Var {
    index: 0usize,
};

pub const Y: Var = Var {
    index: 1usize,
};

pub const Z: Var = Var {
    index: 2usize,
};
