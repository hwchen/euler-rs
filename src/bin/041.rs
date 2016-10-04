// pandigital prime
//
// pandigital for n digits, and is prime.
//
// generate n-pandigitals through permutations.
// Then test for primality.
//
// I should try to implement permutation through Heap's algorithm myself
//
// first pass rust was 0.8s, then rewrote to local digits_to_int
// second pass was 0.7s, not sure if it's really faster.
// hask is 1.8s, also uses permutations so it is slower. but maybe
// rust heap implementation is faster?

extern crate euler;
extern crate permutohedron;

use euler::{is_prime, digits_to_int};
use permutohedron::Heap;

fn main() {
    println!("{}", digits_to_int(&vec![1,2,3]));
    let mut max_prime = 0;

    for n in 3..9+1 {
        let mut digits: Vec<_> = (1..n+1).collect();
        let permutations = Heap::new(&mut digits);

        for permutation_digits in permutations {
            // do this using base 10 next time?
            //let test_num = permutation_digits.iter()
            //    .map(|&x| std::char::from_digit(x as u32, 10).unwrap())
            //    .collect::<String>()
            //    .parse::<usize>().unwrap();

            let test_num = digits_to_int(&permutation_digits);

            if is_prime(test_num) && test_num > max_prime {
                max_prime = test_num;
            }
        }
    }

    println!("max n-pandigital prime: {}", max_prime);
}

