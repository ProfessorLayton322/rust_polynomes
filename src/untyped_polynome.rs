use crate::variables::Var;
use crate::untyped_monome::UntypedMonome;

use std::fmt::Debug;
use std::ops::{Mul, Add};
use std::cmp::Eq;
use num_traits::pow::Pow;
use std::vec::Vec;
use std::convert::From;
use std::default::Default;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UntypedPolynome {
    monomes: Vec<UntypedMonome>,
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

impl Mul<UntypedPolynome> for UntypedPolynome {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            monomes: self.monomes.into_iter().flat_map(|monome| {
                rhs.monomes.iter().map(move |rhs_monome| monome.clone() * rhs_monome.clone())
            }).collect()
        }
    }
}

impl Add<UntypedPolynome> for UntypedPolynome {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            monomes: self.monomes.into_iter().chain(rhs.monomes.into_iter()).collect(),
        }
    }
}

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

