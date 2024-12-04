use crate::traits::Semiring;
use crate::untyped_monome::UntypedMonome;
use crate::variables::Var;

use num_traits::{One, Pow};
use std::cmp::Eq;
use std::collections::HashMap;
use std::convert::Into;
use std::default::Default;
use std::fmt::Debug;
use std::ops::{Mul, Neg};
use std::result::Result;

use duplicate::duplicate_item;

///This struct is a wrapper for coefficients. When ,ultiplied with variables and monomes it results
///in `TypedMonome` or `TypedPolynome`
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Coeff<T: Semiring>(pub T);

/// This struct describes a monome with a fixed type coefficient provided via multiplication with
/// `Coeff`
///
/// # Usage
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
///
/// let monome = Coeff(2u32) * X * X * Y;
///
/// assert_eq!(monome.substitute(vec![
///     (X, 3u32),
///     (Y, 2u32)
/// ]).unwrap(), 36u32);
/// ```
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TypedMonome<T: Semiring> {
    pub coeff: T,
    pub vars: UntypedMonome,
}

/// Coeff to monome
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
///
/// let monome : TypedMonome<f32> = Coeff(2.0f32).into();
/// assert_eq!(monome, TypedMonome {coeff: 2.0f32, vars: UntypedMonome::default()} );
/// ```
impl<T: Semiring> From<Coeff<T>> for TypedMonome<T> {
    fn from(val: Coeff<T>) -> Self {
        Self {
            coeff: val.0,
            vars: UntypedMonome::default(),
        }
    }
}

///This enum type describes errors that can occur when calling `substitue` method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstitutionError {
    ///Returned in case of repeating variables in substitution list
    RepeatingVariable(Var),
    ///Returned in case of missing variables in substitution list that are required for
    ///substitution
    MissingVariable(Var),
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
impl<T: Into<UntypedMonome>, U: Semiring> Mul<T> for Coeff<U> {
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
impl<U: Semiring> Mul<Coeff<U>> for name {
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
impl<U: Semiring> Into<TypedMonome<U>> for name {
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
impl<U: Semiring, T: Into<TypedMonome<U>>> Mul<T> for TypedMonome<U> {
    type Output = TypedMonome<U>;

    fn mul(self, arg: T) -> Self::Output {
        let rhs: TypedMonome<U> = arg.into();
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
impl<U: Semiring> Mul<TypedMonome<U>> for name {
    type Output = TypedMonome<U>;

    fn mul(self, rhs: TypedMonome<U>) -> TypedMonome<U> {
        let lhs: TypedMonome<U> = self.into();
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
impl<U: Semiring> Pow<usize> for TypedMonome<U> {
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
impl<U: Semiring + Neg<Output = U>> Neg for TypedMonome<U> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.coeff = self.coeff.neg();
        self
    }
}

impl<T: Semiring> TypedMonome<T> {
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
    pub fn new(val: T) -> Self {
        Self {
            coeff: val,
            vars: UntypedMonome::default(),
        }
    }

    /// Typed monome substitution
    ///
    /// The type of substitution values must be multiplicable with itself and the coefficient type
    /// and must implement `One` trait
    ///
    /// Returns a substitution error if either the variables are repeated or some of required
    /// variables are missing
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_polynomes::{Coeff, SubstitutionError};
    /// use rust_polynomes::variables::{X, Y};
    /// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
    ///
    /// let monome = Coeff(2u32) * X * Y;
    /// assert_eq!(monome.substitute(vec![
    ///     (X, 3u32),
    ///     (Y, 2u32)
    /// ]).unwrap(), 12u32);
    ///
    /// assert_eq!(monome.substitute(
    ///     vec![(X, 1u32)]
    /// ), Err(SubstitutionError::MissingVariable(Y)));
    ///
    /// assert_eq!(monome.substitute(
    ///     vec![
    ///         (X, 1u32),
    ///         (Y, 1u32),
    ///         (Y, 1u32),
    ///     ]
    /// ), Err(SubstitutionError::RepeatingVariable(Y)));
    /// ```
    pub fn substitute<U: Mul<Output = U> + One + Sized + Clone>(
        &self,
        substitute_list: Vec<(Var, U)>,
    ) -> Result<<T as Mul<U>>::Output, SubstitutionError>
    where
        T: Mul<U>,
    {
        let mut var_map = HashMap::<usize, U>::default();

        for (var, val) in substitute_list.iter() {
            if let Some(_v) = var_map.insert(var.0, val.clone()) {
                return Err(SubstitutionError::RepeatingVariable(*var));
            }
        }

        let mut acc = <U as One>::one();

        for (index, power) in self.vars.powers.iter() {
            let val = if let Some(val) = var_map.get(index) {
                val
            } else {
                return Err(SubstitutionError::MissingVariable(Var(*index)));
            };

            for _ in 0..*power {
                acc = acc * val.clone();
            }
        }

        Ok(self.coeff * acc)
    }

    /// Typed monome substitution
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_polynomes::Coeff;
    /// use rust_polynomes::variables::{X, Y};
    /// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
    ///
    /// let mut monome = Coeff(3u32) * X * Y * X;
    /// assert_eq!(monome.extract_variable(X), 2usize);
    /// assert_eq!(monome, TypedMonome {
    ///     coeff: 3u32,
    ///     vars: UntypedMonome {
    ///         powers: vec![(1, 1)],
    ///     }
    /// })
    /// ```
    pub fn extract_variable(&mut self, var: Var) -> usize {
        let mut counter = 0usize;
        let counter_ref = &mut counter;

        *self = Self {
            coeff: self.coeff,
            vars: UntypedMonome {
                powers: self
                    .vars
                    .powers
                    .iter()
                    .filter_map(|link| {
                        let (index, power) = *link;
                        if index == var.0 {
                            *counter_ref += power;
                            return None;
                        }
                        Some((index, power))
                    })
                    .collect(),
            },
        };

        counter
    }
}
