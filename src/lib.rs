mod variables;
pub use variables::{Var, X, Y, Z};

mod monomes;
pub use monomes::UntypedMonome;

use num_traits::Pow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smth() {
        let pow = X.pow(3);
        let mult = X * X * X;
        assert_eq!(pow, mult);

        assert_eq!(X * Y * Z * Z, Y * X * Z.pow(2));
        assert_ne!(X * X * X, Y * Z * X);
    }
}
