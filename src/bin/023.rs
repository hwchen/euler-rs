extern crate euler;
extern crate itertools;

use euler::proper_divisors;
use itertools::Itertools;
use std::collections::HashMap;

fn is_abundant_number(n:usize) -> bool {
    n < proper_divisors(n).iter().fold(0, |sum, x| sum + x)
}

fn abundant_numbers_below(limit:usize) -> Vec<usize> {
    let mut memo = HashMap::new();

    for i in 12..limit {
        if !memo.contains_key(&i) {
            if is_abundant_number(i) {
                memo.insert(i, 0);
            }
        }
    }

    memo.keys().cloned().collect()
}

fn sums_two_abundant_numbers(limit: usize) -> HashMap<usize, usize> {
    // limit is that of the abundant number, not the sum of two.

    let abundants = abundant_numbers_below(limit);
    let sums = abundants.clone().into_iter().combinations(2)
        .map(|combination| combination.iter().sum());

    // put combinations, and doubled abundants into hashmap
    let mut res = HashMap::new();
    for i in sums {
        res.insert(i, 0);
    }

    for i in abundants {
        res.insert(i*2, 0);
    }

    res
}

fn not_sum_two_abundants() -> Vec<usize> {
    let limit = 28123;
    let sums_abundants = sums_two_abundant_numbers(limit);

    (1..limit).filter(|i| {
        !sums_abundants.contains_key(&i)
    }).collect()
}

fn main() {
//    let n = 11;
//    let limit = 28123;
//    println!("is {} abundant number: {}", n, is_abundant_number(n));
//    println!("abundant numbers below {}: {:?}", limit, abundant_numbers_below(limit));
//    println!("abundant numbers below {}: {:?}", limit, sums_two_abundant_numbers(limit));
    let sum = not_sum_two_abundants().iter().fold(0, |sum, x| sum + x);
    println!("sum of numbers not sum of 2 abundants: {:?}", sum);

}
