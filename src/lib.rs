pub mod variables;

mod traits;

mod untyped_monome;
mod typed_monome;

pub mod monomes {
    pub use crate::untyped_monome::UntypedMonome;
    pub use crate::typed_monome::TypedMonome;
}

pub use typed_monome::Coeff;
pub use typed_monome::SubstitutionError;

mod untyped_polynome;
mod typed_polynome;

pub mod polynomes {
    pub use crate::untyped_polynome::UntypedPolynome;
    pub use crate::typed_polynome::TypedPolynome;
}

#[cfg(test)]
mod tests {
    use super::*;
    use variables::{X, Y, Z};
    use num_traits::Pow;

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
