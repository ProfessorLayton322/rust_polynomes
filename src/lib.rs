mod variables;
pub use variables::{Var, X, Y, Z};

mod untyped_monome;
pub use untyped_monome::UntypedMonome;

mod untyped_polynome;
pub use untyped_polynome::UntypedPolynome;

use num_traits::Pow;

#[cfg(test)]
mod tests {
    use super::*;

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
