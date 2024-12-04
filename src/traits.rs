//! This crate provides a `Semiring` trait that is used as a constraint for coefficient
//! types
//!
//! # Requirements
//!
//! The type needs to meet the properties of a semiring and also be sized and copyable. Then
//! `Semiring` will be automatically implemented for it:
//!
//! - [`num_traits::One`]
//! - [`num_traits::Zero`]
//! - [`std::ops::Add`]
//! - [`std::ops::Mul`]

use num_traits::{One, Zero};
use std::ops::{Add, Mul};

pub trait Semiring: Add<Output = Self> + Mul<Output = Self> + Zero + One + Copy + Sized {}

impl<T> Semiring for T where T: Add<Output = Self> + Mul<Output = Self> + Zero + One + Copy + Sized {}
