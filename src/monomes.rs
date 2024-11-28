use crate::variables::Var;
use std::fmt::Debug;
use std::ops::Mul;
use std::cmp::Eq;
use num_traits::pow::Pow;
use std::vec::Vec;
use std::convert::From;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UntypedMonome {
    //invariant - powers are always sorted and non-repeating by variable
    powers: Vec<(usize, usize)>,
}

impl<const INDEX: usize> From<Var<INDEX>> for UntypedMonome {
    fn from(_var: Var<INDEX>) -> Self {
        Self {
            powers: vec![(INDEX, 1)],
        }
    }
}

impl<const INDEX: usize> Pow<usize> for Var<INDEX> {
    type Output = UntypedMonome;

    fn pow(self, pow: usize) -> Self::Output {
        UntypedMonome {
            powers: vec![(INDEX, pow)],
        }
    }
}

impl Mul<UntypedMonome> for UntypedMonome {
    type Output = Self;

    fn mul(self, rhs: UntypedMonome) -> Self {
        let mut l = 0usize;
        let mut r = 0usize;

        let mut result : Vec<(usize, usize)> = vec![];
        result.reserve(self.powers.len() + rhs.powers.len());

        while l < self.powers.len() && r < rhs.powers.len() {
            if self.powers[l].0 < rhs.powers[r].0 {
                result.push(self.powers[l]);
                l += 1;
            } else if self.powers[l].0 > rhs.powers[r].0 {
                result.push(rhs.powers[r]);
                r += 1;
            } else {
                result.push((self.powers[l].0, self.powers[l].1 + rhs.powers[r].1));
                l += 1;
                r += 1;
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

        Self {
            powers: result,
        }
    }
}

impl<const INDEX: usize> Mul<Var<INDEX>> for UntypedMonome {
    type Output = Self;

    fn mul(self, rhs: Var<INDEX>) -> Self::Output {
        let rhs_monome : UntypedMonome = rhs.into();
        self * rhs_monome
    }
}

impl<const INDEX: usize> Mul<UntypedMonome> for Var<INDEX> {
    type Output = UntypedMonome;

    fn mul(self, rhs: UntypedMonome) -> Self::Output {
        rhs * self
    }
}

impl <const A: usize, const B: usize> Mul<Var<A>> for Var<B> {
    type Output = UntypedMonome;

    fn mul(self, rhs: Var<A>) -> Self::Output {
        let rhs_monome : UntypedMonome = rhs.into();
        self * rhs_monome
    }
}
