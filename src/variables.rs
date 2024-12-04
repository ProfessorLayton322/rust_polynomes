//! This module provides object variables that can be used to describe the polynome
//!
//! It also provides three different ready-to-use variables - `X`, `Y` and `Z`

/// A variable type. The identity of the variable is determined by `usize` index
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Var(pub usize);

pub const X: Var = Var(0usize);

pub const Y: Var = Var(1usize);

pub const Z: Var = Var(2usize);
