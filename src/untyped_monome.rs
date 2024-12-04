use crate::variables::Var;

use num_traits::pow::Pow;
use std::cmp::{Eq, Ordering};
use std::convert::From;
use std::default::Default;
use std::fmt::Debug;
use std::ops::Mul;
use std::vec::Vec;

/// This struct describes a monome in a context where a coefficient with a fixed type was not yet
/// provided
///
/// # Usage
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::UntypedMonome;
///
/// let first_monome = X * Y * X;
/// let second_monome = Y * X * X;
///
/// assert_eq!(first_monome, second_monome);
/// ```
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UntypedMonome {
    //invariant - powers are always sorted and non-repeating by variable
    pub powers: Vec<(usize, usize)>,
}

/// Constructs monome from variable
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{Var, X};
/// use rust_polynomes::monomes::UntypedMonome;
///
/// let monome : UntypedMonome = X.into();
/// assert_eq!(monome, UntypedMonome {powers: vec![(0, 1)]} );
///
/// let another : UntypedMonome = Var(100usize).into();
/// assert_eq!(another, UntypedMonome {powers: vec![(100, 1)]} );
/// ```
impl From<Var> for UntypedMonome {
    fn from(var: Var) -> Self {
        Self {
            powers: vec![(var.0, 1)],
        }
    }
}

/// Raise variable to power
///
/// # Examples
///
/// ```
/// use num_traits::pow::Pow;
/// use rust_polynomes::variables::{Var, X};
/// use rust_polynomes::monomes::UntypedMonome;
///
/// let monome : UntypedMonome = X.pow(4);
/// assert_eq!(monome, UntypedMonome {powers: vec![(0, 4)]} );
/// ```
impl Pow<usize> for Var {
    type Output = UntypedMonome;

    fn pow(self, pow: usize) -> Self::Output {
        UntypedMonome {
            powers: vec![(self.0, pow)],
        }
    }
}

/// Multiplies two monomes
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::UntypedMonome;
///
/// let A : UntypedMonome = X.into();
/// let B : UntypedMonome = Y.into();
/// assert_eq!(A * B, UntypedMonome {powers: vec![(0, 1), (1, 1)]} );
///
/// ```
impl<T: Into<UntypedMonome>> Mul<T> for UntypedMonome {
    type Output = Self;

    fn mul(self, arg: T) -> Self {
        let mut l = 0usize;
        let mut r = 0usize;

        let rhs: UntypedMonome = arg.into();

        let mut result: Vec<(usize, usize)> =
            Vec::with_capacity(self.powers.len() + rhs.powers.len());

        while l < self.powers.len() && r < rhs.powers.len() {
            match self.powers[l].0.cmp(&rhs.powers[r].0) {
                Ordering::Less => {
                    result.push(self.powers[l]);
                    l += 1;
                }
                Ordering::Greater => {
                    result.push(rhs.powers[r]);
                    r += 1;
                }
                Ordering::Equal => {
                    result.push((self.powers[l].0, self.powers[l].1 + rhs.powers[r].1));
                    l += 1;
                    r += 1;
                }
            }
        }

        while l < self.powers.len() {
            result.push(self.powers[l]);
            l += 1;
        }

        while r < rhs.powers.len() {
            result.push(rhs.powers[r]);
            r += 1;
        }

        Self { powers: result }
    }
}

/// Multiplies variable and a monome
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::monomes::UntypedMonome;
///
/// assert_eq!(X * (Y * Z), Z * X * Y);
/// ```
impl<T: Into<UntypedMonome>> Mul<T> for Var {
    type Output = UntypedMonome;

    fn mul(self, rhs: T) -> Self::Output {
        let monome: UntypedMonome = self.into();
        monome * rhs
    }
}

/// Raise monome to power
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::monomes::UntypedMonome;
///
/// use num_traits::pow::Pow;
///
/// let monome = (X * Y * Z).pow(3usize);
/// assert_eq!(monome, X.pow(3usize) * Y.pow(3usize) * Z.pow(3usize));
/// ```
impl Pow<usize> for UntypedMonome {
    type Output = Self;

    fn pow(self, pow: usize) -> Self {
        Self {
            powers: self
                .powers
                .into_iter()
                .map(|factor| (factor.0, factor.1 * pow))
                .collect(),
        }
    }
}
