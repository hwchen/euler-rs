#![feature(conservative_impl_trait)]
// first four consecutive numbers to have four distinct prime factors
//
// Took 0.54s. haskell 3.1s, similar algo

extern crate euler;

use euler::{factors, is_prime};

fn distinct_prime_factors(n:usize) -> impl Iterator<Item=usize> {
    factors(n).into_iter()
        .filter(|&n| is_prime(n))
}

fn check_four(n: usize) -> bool {
    for offset in 0..4 {
        let x = n + offset;
        if distinct_prime_factors(x).count() != 4 {
            return false;
        }
    }
    true
}

fn main() {
    for n in 645.. {
        if check_four(n) {
            println!("first of four consec: {}", n);
            break;
        }
    }

}
