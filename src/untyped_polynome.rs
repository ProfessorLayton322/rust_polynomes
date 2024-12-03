use crate::variables::Var;
use crate::untyped_monome::UntypedMonome;

use std::fmt::Debug;
use std::ops::{Mul, Add};
use std::cmp::Eq;
use std::vec::Vec;
use std::convert::From;
use std::default::Default;

use num_traits::pow::Pow;
use duplicate::duplicate_item;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UntypedPolynome {
    pub monomes: Vec<UntypedMonome>,
}

impl UntypedPolynome {
/// Sort monomes without adding them to each other
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::polynomes::UntypedPolynome;
///
/// let mut A = Z * X + X * Y + Y * Z;
/// A.order();
/// assert_eq!(A, X * Y + X * Z + Y * Z);
/// ```
    pub fn order(&mut self) {
        self.monomes.sort();
    }
}

impl From<Var> for UntypedPolynome {
    fn from(var: Var) -> Self {
        Self {
            monomes: vec![var.into()],
        }
    }
}

impl From<UntypedMonome> for UntypedPolynome {
    fn from(monome: UntypedMonome) -> Self {
        Self {
            monomes: vec![monome],
        }
    }
}

/// Monomes and variables addition into polynomes
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::monomes::UntypedMonome;
/// use rust_polynomes::polynomes::UntypedPolynome;
///
/// let polynome : UntypedPolynome = X * Y + Y * Z;
/// let another = polynome + X;
/// assert_eq!(another, UntypedPolynome {
///     monomes: vec![
///         UntypedMonome {
///             powers: vec![(0, 1), (1, 1)],
///         },
///         UntypedMonome {
///             powers: vec![(1, 1), (2, 1)],
///         },
///         UntypedMonome {
///             powers: vec![(0, 1)],
///         }
///     ],
/// });
/// ```
#[duplicate_item(name; [UntypedPolynome]; [UntypedMonome]; [Var])]
impl<T: Into<UntypedPolynome>> Add<T> for name {
    type Output = UntypedPolynome;

    fn add(self, rhs: T) -> Self::Output {
        let lhs : UntypedPolynome = self.into();
        let rhs_polynome : UntypedPolynome = rhs.into();
        UntypedPolynome {
            monomes: lhs.monomes.into_iter().chain(rhs_polynome.monomes).collect(),
        }
    }
}

/// Multiply polynomes
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::monomes::UntypedMonome;
/// use rust_polynomes::polynomes::UntypedPolynome;
///
/// let mut first = (X + Y + Z) * (X + Y + Z);
/// first.order();
///
/// let mut second = (X * X + Y * Y + Z * Z) + (X * Y + X * Y) + (X * Z + X * Z) + (Y * Z + Y * Z);
/// second.order();
///
/// assert_eq!(first, second);
/// ```
impl<T : Into<UntypedPolynome>> Mul<T> for UntypedPolynome {
    type Output = UntypedPolynome;

    fn mul(self, arg: T) -> Self::Output {
        let rhs : UntypedPolynome = arg.into();
        Self {
            monomes: self.monomes.into_iter().flat_map(|monome| {
                rhs.monomes.iter().map(move |rhs_monome| monome.clone() * rhs_monome.clone())
            }).collect()
        }
    }
}

/// Raise polynom to power 
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::monomes::UntypedMonome;
/// use rust_polynomes::polynomes::UntypedPolynome;
///
/// use num_traits::pow::Pow;
///
/// let mut first = (X + Y).pow(2usize);
/// first.order();
///
/// let mut second = X * X + Y * Y + X * Y + X * Y;
/// second.order();
///
/// assert_eq!(first, second);
///
/// ```
impl Pow<usize> for UntypedPolynome {
    type Output = UntypedPolynome;

    fn pow(self, pow: usize) -> Self::Output {
        if pow == 0 {
            panic!("Can not raise polynome to power of zero");
        }
        let mut answer = self.clone();
        for _ in 0usize..(pow - 1) {
            answer = answer * self.clone();
        }
        answer
    }
}
