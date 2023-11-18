use rug::{Assign, Float, ops::Pow};

const MAX_PRECISION_REQUIRED: u32 = 17;

fn pi_nth_digit(n: u32) -> rug::Integer {
    fn pi_n(n: u32) -> Float {
        let precision = MAX_PRECISION_REQUIRED;
        let two_n = Float::with_val(precision, 2).pow(n);
        let b2n = Float::with_val(precision, -2 * (n as i32)) *
                  Float::zeta(&Float::with_val(precision, 1 - 2 * (n as i32)));
        let nominator = Float::with_val(precision, 2).pow(-1 * (n as i32 + 1)) *
                        Float::factorial(2 * n);
        let denominator = two_n.clone() * b2n * 
                          (1 - two_n.clone().pow(-2 * (n as i32))) *
                          (1 - Float::with_val(precision, 3).pow(-2 * (n as i32))) *
                          (1 - Float::with_val(precision, 5).pow(-2 * (n as i32))) *
                          (1 - Float::with_val(precision, 7).pow(-2 * (n as i32)));
        (nominator / denominator).pow(Float::with_val(precision, 1 / (2 * n)))
    }

    match n {
        0 => rug::Integer::from(3),
        1 => rug::Integer::from(1),
        2 => rug::Integer::from(4),
        _ => {
            let pi_n_minus_1 = pi_n(n - 1);
            let frac_part = (pi_n_minus_1 * Float::with_val(MAX_PRECISION_REQUIRED, 10).pow(n - 1))
                            .fract();
            (frac_part * Float::with_val(MAX_PRECISION_REQUIRED, 10)).floor().to_integer().unwrap()
        }
    }
}

fn main() {
    let pi = Float::with_val(MAX_PRECISION_REQUIRED, Float::constant_pi());
    let pi_str = pi.to_string_radix(10, Some(MAX_PRECISION_REQUIRED as usize));
    for i in 1..=MAX_PRECISION_REQUIRED {
        let expected_digit: u32 = pi_str.chars().nth(i as usize + 1).unwrap().to_digit(10).unwrap();
        let calculated_digit = pi_nth_digit(i);
        assert_eq!(calculated_digit.to_u32().unwrap(), expected_digit,
                   "Mismatch at the {}th digit of pi", i);
    }
    println!("All digits match correctly.");
}