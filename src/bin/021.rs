// amicable numbers
//
// create hash table of computed divisor sums per n. use this
// to look up if d(a) = b and d(b) = a.
//

extern crate euler;

use std::collections::HashMap;
use euler::proper_divisors;

fn sum_proper_divisors(n:usize) -> usize {
    proper_divisors(n).iter().fold(0, |sum, x| sum + x)
}

fn get_amicable_number(a:usize) -> Option<usize> {
    let b = sum_proper_divisors(a);
    if sum_proper_divisors(b) == a && a != b {
        Some(b)
    } else {
        None
    }
}

// how to avoid testing both sides?
fn generate_amicable_numbers_below(limit:usize) -> Vec<usize> {
    let mut memo = HashMap::new();

    for a in 2..limit {
        if !memo.contains_key(&a) {
            if let Some(b) = get_amicable_number(a) {
                memo.insert(a, 0);
                memo.insert(b, 0);
            }
        }
    }

    memo.keys().cloned().collect()
}
fn main() {
//    println!("sum proper divisors 220: {:?}", sum_proper_divisors(220));
//    println!("sum proper divisors 284: {:?}", sum_proper_divisors(284));
//    println!("amicable number of 220: {:?}", get_amicable_number(220));
//    println!("amicable number of 284: {:?}", get_amicable_number(284));

    let limit = 10000;
    let sum_amicable_numbers = generate_amicable_numbers_below(limit)
        .iter()
        .fold(0, |sum, x| sum + x);

    println!("sum amicable numbers below {}: {}", limit, sum_amicable_numbers);
//    println!("amicable numbers below {}: {:?}", limit, generate_amicable_numbers_below(limit));
}
