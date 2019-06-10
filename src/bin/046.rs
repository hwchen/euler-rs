// goldbach conjecture
//
// find smallest odd composite number that can't be written as sum of prime and twice a square.

// somewhere in here there's a limit that crosses
// There are two variables: the prime, and the number squared.
//
// can't just check, need to generate numbers.
//
// rust is 2s, hask is 0.3s. So I really should optimize this.
// oh, hask used limits of 1000 for primes and 100 for the square
// Then rust is faster, 0ms.
//
// I was using limit of 10000 for everything.

extern crate euler;

use euler::{is_prime};
use std::collections::HashSet;

fn main() {
    let odd_composites = (3..).step_by(2).filter(|&x| !is_prime(x));

    let mut cache = HashSet::new();

    for a in (1..).filter(|&x| is_prime(x)).take(1000) {
        for b in 1usize..100 {
            cache.insert(a + 2 * b.pow(2));
        }
    }

    for n in odd_composites {
        if !cache.contains(&n) {
            println!("smallest odd composite not by goldbach: {}", n);
            break;
        }
    }
}
