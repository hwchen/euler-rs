extern crate euler;

use euler::{is_prime, factors};
use std::cmp::max;

fn main() {
    let max_prime_factor = factors(600851475143).into_iter()
        .filter(|&i| {
            is_prime(i)
        }).fold(0, |z, i| {
            max(z, i)
        });

    println!("Greatest prime factor of 600851475143: {}", max_prime_factor);
}
