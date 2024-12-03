use crate::variables::Var;
use crate::untyped_monome::UntypedMonome;
use crate::traits::CommutativeSemiring;

use std::fmt::Debug;
use std::ops::{Mul, Neg};
use num_traits::{One, Pow};
use std::cmp::Eq;
use std::convert::Into;
use std::default::Default;

use duplicate::duplicate_item;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Coeff<T: CommutativeSemiring> (pub T);

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TypedMonome<T: CommutativeSemiring> {
    pub coeff: T,
    pub vars: UntypedMonome,
}

/// Typed monome constructor
///
/// # Examples
///
/// ```
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
///
/// let typed = TypedMonome::<f32>::new(3.0f32);
/// assert_eq!(typed.coeff, 3.0f32);
/// assert_eq!(typed.vars, UntypedMonome::default() );
/// ```
impl<T: CommutativeSemiring> TypedMonome<T> {
    pub fn new(val: T) -> Self {
        Self {
            coeff: val,
            vars: UntypedMonome::default(),
        }
    }
}

/// Coefficient to monome
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::{Coeff, monomes::TypedMonome};
///
/// let typed : TypedMonome<f32> = Coeff(3.0f32).into();
/// assert_eq!(typed, TypedMonome::<f32>::new(3.0f32));
/// ```
impl<T: CommutativeSemiring> Into<TypedMonome<T>> for Coeff<T> {
    fn into(self) -> TypedMonome<T> {
        TypedMonome {
            coeff: self.0,
            vars: UntypedMonome::default(),
        }
    }
}

/// Multiplication of variables and coefficients
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// let monome = Coeff(3.0f32) * (X * Y * Z);
/// assert_eq!(monome.coeff, 3.0f32);
/// assert_eq!(monome.vars, UntypedMonome {powers: vec![(0, 1), (1, 1), (2, 1)]} );
/// ```
impl<T: Into<UntypedMonome>, U : CommutativeSemiring> Mul<T> for Coeff<U> {
    type Output = TypedMonome<U>;

    fn mul(self, arg: T) -> Self::Output {
        TypedMonome {
            coeff: self.0,
            vars: arg.into(),
        }
    }
}

/// Multiplication of variables and coefficients
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// let monome = (X * Y * Z) * Coeff(3.0f32);
/// assert_eq!(monome.coeff, 3.0f32);
/// assert_eq!(monome.vars, UntypedMonome {powers: vec![(0, 1), (1, 1), (2, 1)]} );
/// ```
#[duplicate_item(name; [UntypedMonome]; [Var])]
impl<U : CommutativeSemiring> Mul<Coeff<U>> for name {
    type Output = TypedMonome<U>;

    fn mul(self, arg: Coeff<U>) -> Self::Output {
        arg * self
    }
}
 
/// Multiplication of variables and coefficients
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// let first : TypedMonome<f32> = X.into();
/// assert_eq!(first.coeff, 1.0f32);
/// assert_eq!(first.vars, UntypedMonome { powers : vec![(0, 1)] } );
///
/// let second : TypedMonome<f32> = (X * Y).into();
/// assert_eq!(second.coeff, 1.0f32);
/// assert_eq!(second.vars, UntypedMonome { powers : vec![(0, 1), (1, 1)] } );
/// ```
#[duplicate_item(name; [UntypedMonome]; [Var])]
impl<U : CommutativeSemiring> Into<TypedMonome<U>> for name {
    fn into(self) -> TypedMonome<U> {
        TypedMonome {
            coeff: <U as One>::one(),
            vars: self.into(),
        }
    }
}

/// Multiplication of two typed monomes
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// let first = Coeff(3.0f32) * X * Y;
/// let second = Coeff(2.0f32) * X * Z;
/// let third = Coeff(6.0f32) * X * X * Y * Z;
///
/// assert_eq!(first * second, third);
/// ```
impl<U: CommutativeSemiring, T: Into<TypedMonome<U>>> Mul<T> for TypedMonome<U> {
    type Output = TypedMonome<U>;

    fn mul(self, arg: T) -> Self::Output {
        let rhs : TypedMonome<U> = arg.into();
        TypedMonome {
            coeff: self.coeff * rhs.coeff,
            vars: self.vars * rhs.vars,
        }
    }
}

/// Multiplication of variables and coefficients
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// let first : TypedMonome<f32> = X.into();
/// let second = (X * Y) * first.clone();
///
/// assert_eq!(second.coeff, 1.0f32);
/// assert_eq!(second.vars, UntypedMonome {powers: vec![(0, 2), (1, 1)]} );
///
/// let third = X * first;
/// assert_eq!(third.coeff, 1.0f32);
/// assert_eq!(third.vars, UntypedMonome {powers: vec![(0, 2)]} );
/// ```
#[duplicate_item(name; [UntypedMonome]; [Var])]
impl<U: CommutativeSemiring> Mul<TypedMonome<U>> for name {
    type Output = TypedMonome<U>;

    fn mul(self, rhs: TypedMonome<U>) -> TypedMonome<U> {
        let lhs : TypedMonome<U> = self.into();
        lhs * rhs
    }
}

/// Raise typed monome to power
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// use num_traits::pow::Pow;
///
/// let monome = (Coeff(2.0f32) * X * Y).pow(3usize);
///
/// assert_eq!(monome.coeff, 8.0f32);
/// assert_eq!(monome.vars, UntypedMonome {powers: vec![(0, 3), (1, 3)]} );
/// ```
impl<U: CommutativeSemiring> Pow<usize> for TypedMonome<U> {
    type Output = Self;

    fn pow(self, pow: usize) -> Self::Output {
        let mut c = <U as One>::one();
        for _ in 0..pow {
            c = c * self.coeff;
        }
        TypedMonome {
            coeff: c,
            vars: self.vars.pow(pow),
        }
    }
}

/// Negative typed monome
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::{Coeff, monomes::{UntypedMonome, TypedMonome}};
///
/// use num_traits::pow::Pow;
///
/// let monome = Coeff(2.0f32) * X;
/// let negative = -monome;
///
/// assert_eq!(negative.coeff, -2.0f32);
/// assert_eq!(negative.vars, UntypedMonome {powers: vec![(0, 1)] } );
/// ```
impl<U: CommutativeSemiring + Neg<Output = U>> Neg for TypedMonome<U> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.coeff = self.coeff.neg();
        self
    }
}
