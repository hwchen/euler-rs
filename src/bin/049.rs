// three primes, each permutations, each four digits
//
// to construct:
//
// - all combinations of choosing 3 from pool of 4-digit prime numbers
// - group into primes with same digits
// - check which ones are same distance and have 3 in seq
//
// in rust, 0.00s, in hask, 0.97. I think hask uses permutations, instead
// of grouping primes by sorted digits. I think that the rust algorithm is
// much faster, so it's probably not the language.

extern crate euler;
extern crate itertools;

use euler::{to_digits, is_prime};
use itertools::Itertools;
use std::collections::HashMap;


fn main() {
    let mut res = vec![];

    let four_digit_primes = (1000..10000).filter(|&x| is_prime(x));

    let primes_with_ordered_digits = four_digit_primes.map(|x| {
        let mut digits = to_digits(x);
        digits.sort();

        (x, digits)
    });

    // Here group together primes which have same digits
    let mut seqs = HashMap::new();
    for (n, digits) in primes_with_ordered_digits {
        let seq = seqs.entry(digits).or_insert(vec![]);
        seq.push(n);
    }

    // filter out sequences that have less than 3 numbers
    let filtered_seqs: Vec<Vec<_>> = seqs.values().cloned()
        //.inspect(|seq| println!("{:?}", seq))
        .filter(|seq| {
            seq.len() >= 3
        }).collect();

    // for each seq, check the combinations
    for seq in filtered_seqs.into_iter() {
        for combination in seq.into_iter().combinations(3) {
            if (combination[0] as isize - combination[1] as isize).abs() ==
                (combination[1] as isize - combination[2] as isize).abs() {

                res.push(combination);
            }
        }
    }

    println!("result seqs: {:?}", res[0].iter().map(|n| n.to_string()).join(""));
    println!("result seqs: {:?}", res[1].iter().map(|n| n.to_string()).join(""));
}
