use crate::variables::Var;

use std::fmt::Debug;
use std::ops::Mul;
use std::cmp::Eq;
use num_traits::pow::Pow;
use std::vec::Vec;
use std::convert::From;
use std::default::Default;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UntypedMonome {
    //invariant - powers are always sorted and non-repeating by variable
    powers: Vec<(usize, usize)>,
}

impl From<Var> for UntypedMonome {
    fn from(var: Var) -> Self {
        Self {
            powers: vec![(var.index, 1)],
        }
    }
}

impl Pow<usize> for Var {
    type Output = UntypedMonome;

    fn pow(self, pow: usize) -> Self::Output {
        UntypedMonome {
            powers: vec![(self.index, pow)],
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

impl Mul<Var> for UntypedMonome {
    type Output = Self;

    fn mul(self, rhs: Var) -> Self::Output {
        let rhs_monome : UntypedMonome = rhs.into();
        self * rhs_monome
    }
}

impl Mul<UntypedMonome> for Var {
    type Output = UntypedMonome;

    fn mul(self, rhs: UntypedMonome) -> Self::Output {
        rhs * self
    }
}

impl Mul<Var> for Var {
    type Output = UntypedMonome;

    fn mul(self, rhs: Var) -> Self::Output {
        let rhs_monome : UntypedMonome = rhs.into();
        self * rhs_monome
    }
}
