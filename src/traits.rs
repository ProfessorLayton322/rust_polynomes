use std::ops::{Add, Mul};
use num_traits::{Zero, One};

pub trait CommutativeSemiring: Add<Output = Self> + Mul<Output = Self> + Zero + One + Copy + Sized {}

impl<T> CommutativeSemiring for T where T: Add<Output = Self> + Mul<Output = Self> + Zero + One + Copy + Sized {}
