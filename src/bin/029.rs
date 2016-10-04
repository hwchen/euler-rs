// distinct powers
//
// generate combinations of and b, then raise a^b, then check for
// overlaps. Just insert to hashmap and count.
//
// Brute force is ok, how to cut down search space?
//
// Oops, needs to use bignum multiplication

// Not bad, my ugly bignum implementation is as fast as haskell's
// native Integer. I still think my bignum can be faster, because
// I make allocations to BigNum for addition during multiplication
//
// .58 seconds when allocating to BigNum for addition in mul.
// weird, just using vecs for addition in mul is possibly even slower?

extern crate euler;

use std::collections::HashSet;
use euler::bignum::BigNum;

fn main() {
    let mut distinct_powers = HashSet::new();

    for a in 2..101 {
        for b in 2..101 {
            let product = format!("{}", a).parse::<BigNum>().unwrap().pow(b);
            distinct_powers.insert(product);
        }
    }

    let res = distinct_powers.len();

//    for n in &distinct_powers {
//        println!("{}", n);
//    }
    println!("distinct powers: {}", res);
}
