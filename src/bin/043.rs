// sub-string pandigital
//
// Test to see if permutation or filter is faster for getting pandigitals
//
// Permutation way faster. Should have been obvious in hindsight?

// rust in 0.16s, hask in 29.48s. Hask seems to use permutations too,
// Not sure why so slow?

extern crate euler;
extern crate permutohedron;

use euler::digits_to_int;
use permutohedron::Heap;

fn substring_tests(digits: &[usize]) -> bool {
    digits_to_int(&digits[1..=3]) % 2 == 0 &&
    digits_to_int(&digits[2..=4]) % 3 == 0 &&
    digits_to_int(&digits[3..=5]) % 5 == 0 &&
    digits_to_int(&digits[4..=6]) % 7 == 0 &&
    digits_to_int(&digits[5..=7]) % 11 == 0 &&
    digits_to_int(&digits[6..=8]) % 13 == 0 &&
    digits_to_int(&digits[7..=9]) % 17 == 0 
}

fn main() {
    let mut digits: Vec<_> = (0..=9).collect();
    let pandigitals = Heap::new(&mut digits);

    let sum_sub_pandigitals: usize = pandigitals
        .filter(|ref pandigital| substring_tests(&pandigital))
        .map(|pandigital| digits_to_int(&pandigital))
        .sum();

    println!("Sum substring pandigitals: {}", sum_sub_pandigitals);
}
