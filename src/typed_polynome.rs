use crate::variables::Var;
use crate::untyped_monome::UntypedMonome;
use crate::untyped_polynome::UntypedPolynome;
use crate::typed_monome::{Coeff, TypedMonome, SubstitutionError};
use crate::traits::CommutativeSemiring;

use std::fmt::Debug;
use std::ops::{Add, Mul, Sub, Neg};
use num_traits::{Zero, One, Pow};
use std::cmp::Eq;
use std::convert::From;
use std::default::Default;

use duplicate::duplicate_item;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TypedPolynome<T: CommutativeSemiring> {
    pub monomes: Vec<TypedMonome<T>>,
}

impl<U: CommutativeSemiring> Default for TypedPolynome<U> {
/// Default polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::polynomes::TypedPolynome;
///
/// let polynome = TypedPolynome::<f32>::default();
/// assert_eq!(polynome.monomes.len(), 1usize);
/// assert_eq!(polynome.monomes[0].coeff, 0.0f32);
/// assert_eq!(polynome.monomes[0].vars.powers.len(), 0);
/// ```
    fn default() -> Self {
        Self {
            monomes: vec![
                TypedMonome::<U>::new(<U as Zero>::zero())
            ],
        }
    }
}

impl<U: CommutativeSemiring> Zero for TypedPolynome<U> {
/// Zero polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::polynomes::TypedPolynome;
/// use num_traits::Zero;
///
///
/// let polynome = TypedPolynome::<f32>::default();
/// assert_eq!(polynome.monomes.len(), 1usize);
/// assert_eq!(polynome.monomes[0].coeff, 0.0f32);
/// assert_eq!(polynome.monomes[0].vars.powers.len(), 0);
/// ```
    fn zero() -> Self {
        Self::default()
    }

/// Check that all coefficients are zero
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::polynomes::TypedPolynome;
/// use num_traits::Zero;
///
/// assert!((Coeff(0u32) * X + Coeff(0u32) * Y).is_zero());
/// assert!(!TypedPolynome::<u32>::new(2u32).is_zero());
/// ```
    fn is_zero(&self) -> bool {
        self.monomes.iter().map(|monome| monome.coeff.is_zero()).fold(true, |acc, x| acc & x)
    }
}

impl<T: CommutativeSemiring> TypedPolynome<T> {
/// Constant polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::polynomes::TypedPolynome;
///
/// let polynome = TypedPolynome::<f32>::new(6.5f32);
/// assert_eq!(polynome.monomes.len(), 1usize);
/// assert_eq!(polynome.monomes[0].coeff, 6.5f32);
/// assert_eq!(polynome.monomes[0].vars.powers.len(), 0);
/// ```
    pub fn new(val: T) -> Self {
        Self {
            monomes: vec![
                TypedMonome::<T>::new(val)
            ],
        }
    }

/// Typed monome substitution 
///
/// # Examples
///
/// ```
/// use rust_polynomes::{Coeff, SubstitutionError};
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome = Coeff(2u32) * X + Coeff(3u32) * Y;
/// assert_eq!(polynome.substitute(vec![
///     (X, 3u32),
///     (Y, 2u32)
/// ]).unwrap(), 12u32);
///
/// assert_eq!(polynome.substitute(
///     vec![(X, 1u32)]
/// ), Err(SubstitutionError::MissingVariable(Y)));
///
/// assert_eq!(polynome.substitute(
///     vec![
///         (X, 1u32),
///         (Y, 1u32),
///         (Y, 1u32),
///     ]
/// ), Err(SubstitutionError::RepeatingVariable(Y)));
/// ```
    pub fn substitute<U: Mul<Output=U> + One + Sized + Clone>(&self, substitute_list: Vec<(Var, U)>) 
    -> Result<<T as Mul<U>>::Output, SubstitutionError>
    where
        T: Mul<U>,
        <T as Mul<U>>::Output : Add<Output = <T as Mul<U>>::Output> + Zero,
    {
        let mut acc = <<T as Mul<U>>::Output as Zero>::zero();

        for monome in self.monomes.iter() {
            acc = acc + monome.substitute(substitute_list.clone())?;
        }

        Ok(acc)
    }

/// Typed monome substitution 
///
/// # Examples
///
/// ```
/// use rust_polynomes::{Coeff, SubstitutionError};
/// use rust_polynomes::variables::{X, Y, Z};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
/// use num_traits::pow::Pow;
///
/// let polynome = Coeff(2u32) * X * Y + Coeff(3u32) * Y;
///
/// let mut first = polynome.clone();
/// first.substitute_polynome(Y, Z.into());
/// assert_eq!(first.monomes, vec![
///     TypedMonome {
///         coeff: 2u32,
///         vars: X * Z,
///     },
///     TypedMonome {
///         coeff: 3u32,
///         vars: Z.into(),
///     }
/// ]);
///
/// let reserve = first.clone();
/// first.substitute_polynome(Y, Z.into());
/// assert_eq!(first, reserve);
///
/// let mut second : TypedPolynome<i32> = X.pow(3).into();
/// second.substitute_polynome(X, (X + Y).into());
/// let mut expected = X.pow(3) + Coeff(3i32) * (X * X * Y + X * Y * Y) + Y.pow(3);
/// expected.order();
///
/// assert_eq!(second, expected);
/// ```
    pub fn substitute_polynome(&mut self, var: Var, polynome: TypedPolynome<T>) where
    {
        *self = self.monomes.iter().map(|monome| {
            let mut clone = monome.clone();
            let pow = clone.extract_variable(var.clone());

            if pow == 0 {
                return clone.into();
            }

            clone * polynome.clone().pow(pow)
        }).fold(TypedPolynome::<T>::zero(), |acc, x| acc + x);
        self.order();
    }

/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
/// use num_traits::pow::Pow;
///
/// let mut polynome = X + Y - (Coeff(1i32) * X).into() + Coeff(3i32) * Y;
/// polynome.order();
/// 
/// assert_eq!(polynome.monomes, vec![
///     TypedMonome {
///         coeff: 4i32,
///         vars: Y.into(),
///    } 
/// ]);
///
/// let mut another = (Coeff(1u32) * X + Y).pow(2);
/// another.order();
/// assert_eq!(another.monomes, vec![
///     TypedMonome {
///         coeff: 2u32,
///         vars: X * Y,
///     },
///     TypedMonome {
///         coeff: 1u32,
///         vars: X * X,
///     },
///     TypedMonome {
///         coeff: 1u32,
///         vars: Y * Y,
///     } 
/// ]);
/// ```
    pub fn order(&mut self) {
        if self.is_zero() {
            self.set_zero();
            return ();
        }
        self.monomes.sort_by(|a, b| a.vars.cmp(&b.vars));
        *self = Self {
            monomes: self.monomes.iter().fold(vec![], |mut acc, monome| {
                let n = acc.len();
                if n == 0 {
                    acc.push(monome.clone());
                    return acc;
                }
                if acc[n - 1].vars == monome.vars {
                    acc[n - 1].coeff = acc[n - 1].coeff + monome.coeff;
                } else {
                    acc.push(monome.clone());
                }
                acc
            }).into_iter().filter(|monome| !monome.coeff.is_zero()).collect(),
        };
        if self.monomes.len() == 0 {
            self.set_zero();
        }
    }
}

impl<U: CommutativeSemiring> One for TypedPolynome<U> {
/// One polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::polynomes::TypedPolynome;
/// use num_traits::One;
///
///
/// let polynome = TypedPolynome::<f32>::one();
/// assert_eq!(polynome.monomes.len(), 1usize);
/// assert_eq!(polynome.monomes[0].coeff, 1.0f32);
/// assert_eq!(polynome.monomes[0].vars.powers.len(), 0);
/// ```
    fn one() -> Self {
        Self::new(<U as One>::one())
    }
}

/// Typed monome constructor
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::X;
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::TypedPolynome;
///
/// let polynome : TypedPolynome<f32> = (Coeff(3.0f32) * X).into();
///
/// assert_eq!(polynome.monomes, vec![
///     TypedMonome {
///         coeff : 3.0f32, 
///         vars: X.into(),
///     } 
/// ]);
///
/// let another : TypedPolynome<u32> = Coeff(2u32).into();
///
/// assert_eq!(another.monomes, vec![
///     TypedMonome {
///         coeff: 2u32,
///         vars: UntypedMonome::default(),
///     }
/// ]);
/// ```
#[duplicate_item(name; [Coeff<U>]; [TypedMonome<U>])]
impl<U: CommutativeSemiring> From<name> for TypedPolynome<U> {
    fn from(val: name) -> Self {
        Self {
            monomes: vec![val.into()],
        }
    }
}

/// Typed monome constructor from untyped
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let first : TypedPolynome<f32> = X.into();
/// assert_eq!(first.monomes, vec![
///     TypedMonome {
///         coeff : 1.0f32, 
///         vars: X.into(),
///     } 
/// ]);
///
/// let second : TypedPolynome<f32> = (X * Y).into();
/// assert_eq!(second.monomes, vec![
///     TypedMonome {
///         coeff : 1.0f32, 
///         vars: X * Y,
///     } 
/// ]);
///
/// let third: TypedPolynome<f32> = (X + Y).into();
/// assert_eq!(third.monomes, vec![
///     TypedMonome {
///         coeff : 1.0f32, 
///         vars: X.into(),
///     },
///     TypedMonome {
///         coeff : 1.0f32, 
///         vars: Y.into(),
///     }
/// ]);
/// ```
impl <U: CommutativeSemiring, T: Into<UntypedPolynome>> From<T> for TypedPolynome<U> {
    fn from(val: T) -> Self {
        //untyped polynome is always non empty
        let untyped : UntypedPolynome = val.into();
        Self {
            monomes: untyped.monomes.into_iter().map(|monome| {
                TypedMonome {
                    coeff: <U as One>::one(),
                    vars: monome,
                }
            }).collect()
        }
    }
}

/// Addition of typed monomes and polynomes create typed polynomes
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome = Coeff(3.0f32) * Y + X;
/// assert_eq!(polynome.monomes, vec![
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: Y.into(),
///     },
///     TypedMonome {
///         coeff: 1.0f32,
///         vars: X.into(),
///     }
/// ]);
///
/// let another = polynome + X * Y + (X + Y);
/// assert_eq!(another.monomes, vec![
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: Y.into(),
///     },
///     TypedMonome {
///         coeff: 1.0f32,
///         vars: X.into(),
///     },
///     TypedMonome {
///         coeff: 1.0f32,
///         vars: X * Y,
///     },
///     TypedMonome {
///         coeff: 1.0f32,
///         vars: X.into(),
///     },
///     TypedMonome {
///         coeff: 1.0f32,
///         vars: Y.into(),
///     }
/// ]);
/// ```
#[duplicate_item(name; [TypedPolynome<U>]; [TypedMonome<U>]; [Coeff<U>])]
impl <U: CommutativeSemiring, T: Into<TypedPolynome<U>>> Add<T> for name {
    type Output = TypedPolynome<U>;

    fn add(self, arg: T) -> TypedPolynome<U> {
        let lhs : TypedPolynome<U> = self.into();
        let rhs : TypedPolynome<U> = arg.into();
        TypedPolynome {
            monomes: lhs.monomes.into_iter().chain(rhs.monomes).collect(),
        }
    }
}

/// Untyped + typed addition create typed polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome = X + Coeff(3.0f32) * Y ;
/// assert_eq!(polynome.monomes, vec![
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: Y.into(),
///     },
///     TypedMonome {
///         coeff: 1.0f32,
///         vars: X.into(),
///     }
/// ]);
/// ```
#[duplicate_item(
    typed untyped;
    duplicate!{
        [
            typed_nested; [Coeff<U>]; [TypedMonome<U>]; [TypedPolynome<U>]
        ]
        [typed_nested] [Var];
        [typed_nested] [UntypedMonome];
        [typed_nested] [UntypedPolynome];
    }
)]
impl<U: CommutativeSemiring> Add<typed> for untyped {
    type Output = TypedPolynome<U>;

    fn add(self, arg: typed) -> Self::Output {
        arg + self
    }
}

/// Multiply typed polynomes
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::Coeff;
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let first : TypedPolynome<f32> = (Coeff(3.0f32) * X).into();
/// let second : TypedPolynome<f32> = (Coeff(2.0f32) * Y).into();
/// 
/// let third = first * second;
///
/// assert_eq!(third.monomes, vec![
///     TypedMonome {
///         coeff: 6.0f32,
///         vars: X * Y,
///     }
/// ]);
/// ```
impl<U: CommutativeSemiring, T: Into<TypedPolynome<U>>> Mul<T> for TypedPolynome<U> {
    type Output = TypedPolynome<U>;

    fn mul(self, arg: T) -> Self::Output {
        let rhs : TypedPolynome<U> = arg.into();
        TypedPolynome {
            monomes: self.monomes.into_iter().flat_map(|monome| {
                rhs.monomes.iter().map(move |rhs_monome| monome.clone() * rhs_monome.clone())
            }).collect(),
        }
    }
}

/// Multiply typed polynomes with other types
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::Coeff;
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let first : TypedPolynome<f32> = (Coeff(3.0f32) * X).into();
/// let second = (X + Y) * first;
///
/// assert_eq!(second.monomes, vec![
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: X * X,
///     },
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: X * Y,
///     }
/// ]);
/// ```
#[duplicate_item(name; [Var]; [UntypedMonome]; [UntypedPolynome]; [Coeff<U>]; [TypedMonome<U>])]
impl<U: CommutativeSemiring> Mul<TypedPolynome<U>> for name {
    type Output = TypedPolynome<U>;

    fn mul(self, arg: TypedPolynome<U>) -> Self::Output {
        arg * self
    }
}

/// Multiply typed monome and untyped polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::Coeff;
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome = (X + Y) * (Coeff(3.0f32) * X);
///
/// assert_eq!(polynome.monomes, vec![
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: X * X,
///     },
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: X * Y,
///     }
/// ]);
///
/// let another = (X + Y) * Coeff(3u32);
/// assert_eq!(another.monomes, vec![
///     TypedMonome {
///         coeff: 3u32,
///         vars: X.into(),
///     },
///     TypedMonome {
///         coeff: 3u32,
///         vars: Y.into(),
///     }
/// ]);
/// ```
#[duplicate_item(name; [Coeff<U>]; [TypedMonome<U>])]
impl<U: CommutativeSemiring> Mul<name> for UntypedPolynome {
    type Output = TypedPolynome<U>;

    fn mul(self, arg: name) -> Self::Output {
        let rhs : TypedMonome<U> = arg.into();
        TypedPolynome {
            monomes: self.monomes.into_iter().map(|monome| monome * rhs.clone()).collect(),
        }
    }
}

/// Multiply typed monome and untyped polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::Coeff;
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome = (Coeff(3.0f32) * X) * (X + Y);
///
/// assert_eq!(polynome.monomes, vec![
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: X * X,
///     },
///     TypedMonome {
///         coeff: 3.0f32,
///         vars: X * Y,
///     }
/// ]);
///
/// let another = Coeff(3u32) * (X + Y);
/// assert_eq!(another.monomes, vec![
///     TypedMonome {
///         coeff: 3u32,
///         vars: X.into(),
///     },
///     TypedMonome {
///         coeff: 3u32,
///         vars: Y.into(),
///     }
/// ]);
/// ```
#[duplicate_item(name; [Coeff<U>]; [TypedMonome<U>])]
impl<U: CommutativeSemiring> Mul<UntypedPolynome> for name {
    type Output = TypedPolynome<U>;

    fn mul(self, rhs: UntypedPolynome) -> Self::Output {
        rhs * self
    }
} 

/// Raise polynom to power 
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// use num_traits::pow::Pow;
///
/// let polynome = Coeff(2u32) * X + Y * Coeff(3u32);
/// assert_eq!(polynome.pow(2).monomes, vec![
///     TypedMonome {
///         coeff: 4u32,
///         vars: X * X,
///     },
///     TypedMonome {
///         coeff: 6u32,
///         vars: X * Y,
///     },
///     TypedMonome {
///         coeff: 6u32,
///         vars: X * Y,
///     },
///     TypedMonome {
///         coeff: 9u32,
///         vars: Y * Y,
///     },
/// ]);
/// ```
impl<U: CommutativeSemiring> Pow<usize> for TypedPolynome<U> {
    type Output = TypedPolynome<U>;

    fn pow(self, pow: usize) -> Self::Output {
        (0..pow).map(|_| self.clone()).fold(<Self as One>::one(), |acc, x| acc * x)
    }
}

/// Negative polynome
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// use num_traits::pow::Pow;
///
/// let polynome : TypedPolynome<i32> = (Coeff(2i32) * X).into();
/// assert_eq!((-polynome).monomes, vec![
///     TypedMonome {
///         coeff: -2i32,
///         vars: X.into(),
///     }
/// ]);
/// ```
impl<U: CommutativeSemiring + Neg<Output=U>> Neg for TypedPolynome<U> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            monomes: self.monomes.into_iter().map(|monome| monome.neg()).collect(),
        }
    }
}

/// Polynomes subtraction
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome = Coeff(2i32) * X + Coeff(1i32) * Y;
/// assert_eq!(polynome - X - X * Y, TypedPolynome {
///     monomes: vec![
///         TypedMonome {
///             coeff: 2i32,
///             vars: X.into(),
///         },
///         TypedMonome {
///             coeff: 1i32,
///             vars: Y.into(),
///         },
///         TypedMonome {
///             coeff: -1i32,
///             vars: X.into(),
///         },
///         TypedMonome {
///             coeff: -1i32,
///             vars: X * Y,
///         }
///     ],
/// })
/// ```
impl<U: CommutativeSemiring + Neg<Output=U>, T : Into<TypedPolynome<U>>> Sub<T> for TypedPolynome<U> {
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        let rhs : TypedPolynome<U> = other.into();
        self + (-rhs)
    }
}

/// Polynomes subtraction
///
/// # Examples
///
/// ```
/// use rust_polynomes::Coeff;
/// use rust_polynomes::variables::{X, Y};
/// use rust_polynomes::monomes::{UntypedMonome, TypedMonome};
/// use rust_polynomes::polynomes::{UntypedPolynome, TypedPolynome};
///
/// let polynome : TypedPolynome<i32> = (X + Y).into();
/// assert_eq!(Y - polynome, TypedPolynome {
///     monomes: vec![
///         TypedMonome {
///             coeff: 1i32,
///             vars: Y.into(),
///         },
///         TypedMonome {
///             coeff: -1i32,
///             vars: X.into(),
///         },
///         TypedMonome {
///             coeff: -1i32,
///             vars: Y.into(),
///         }
///     ],
/// });
/// ```
#[duplicate_item(name; [Var]; [UntypedMonome]; [UntypedPolynome]; [TypedMonome<U>]; [Coeff<U>])]
impl<U: CommutativeSemiring + Neg<Output=U>> Sub<TypedPolynome<U>> for name {
    type Output = TypedPolynome<U>;

    fn sub(self, other: TypedPolynome<U>) -> Self::Output {
        let lhs : TypedPolynome<U> = self.into();
        lhs - other
    }
}
