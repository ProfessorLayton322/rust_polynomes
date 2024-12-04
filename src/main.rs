use rust_polynomes::variables::{X, Y, Z};
use rust_polynomes::Coeff;

fn main() {
    let polynome = Coeff(2u32) * X + Y;
    let result = polynome.substitute(vec![
        (X, 3u32),
        (Y, 4u32)
    ]).unwrap();

    println!("Result is {}", result);

    let mut another = polynome.clone();
    another.substitute_polynome(Y, (X + Z).into());

    let another_result = another.substitute(vec![
        (X, 2u32),
        (Z, 3u32)
    ]).unwrap();

    println!("Another result is {}", another_result);
}
