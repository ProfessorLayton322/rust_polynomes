//! This crate provides a framework for working with polynomes
//!
//! The most important structure is `polynomes::TypedPolynome`, the rest of the crate are helpers
//! and wrappers to support it
//!
//! # Usage
//!
//! ```
//! use rust_polynomes::{Coeff};
//! use rust_polynomes::variables::{X, Y};
//! use rust_polynomes::polynomes::TypedPolynome;
//!
//! let polynome = Coeff(2u32) * (X + Y) + X * Y;
//! assert_eq!(polynome.substitute(vec![
//!     (X, 2u32),
//!     (Y, 4u32)
//! ]).unwrap(), 20u32);
//! ```
//!
//! # Custom coefficient data types
//!
//! You can use this crate with custom types of your own - all you need to do is meet the
//! requirements for `Semiring` trait described in `traits` module

pub mod variables;

pub mod traits;

mod typed_monome;
mod untyped_monome;

/// This module provides structs to describe monomes
pub mod monomes {
    pub use crate::typed_monome::TypedMonome;
    pub use crate::untyped_monome::UntypedMonome;
}

pub use typed_monome::Coeff;
pub use typed_monome::SubstitutionError;

mod typed_polynome;
mod untyped_polynome;

/// This module provides structs to describe polynomes
pub mod polynomes {
    pub use crate::typed_polynome::TypedPolynome;
    pub use crate::untyped_polynome::UntypedPolynome;
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::Pow;
    use variables::{X, Y, Z};

    #[test]
    fn untyped_monomes() {
        let pow = X.pow(3);
        let mult = X * X * X;
        assert_eq!(pow, mult);

        assert_eq!(X * Y * Z * Z, Y * X * Z.pow(2));
        assert_ne!(X * X * X, Y * Z * X);

        let a = X * X;
        assert_eq!(X * a, X * X * X);
    }
}
