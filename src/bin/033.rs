#![feature(conservative_impl_trait)]
// cancelling fractions

// numerator and denominator are 2 digits long. cancel the second should equal
// original fraction.
// Does not include the trivial example of eliminating 0.

// Should I generate the fractions from before or after simplification?

// The cancelled fractions can be:
// - first digits
// - diag digits
// - diag digits
// - last digits

// generating is easier than trying to figure out what to cancel.

extern crate itertools;

use itertools::Itertools;

// First, generate all combinations of 1-digit fractions
pub fn generate_one_digit_fractions() -> impl Iterator<Item=(usize, usize)> {
    (1usize..10).cartesian_product(1usize..10)
        .filter(|&(a,b)| a < b) //can be more efficient than filter?
}

// Helper function for adding a digit to front
pub fn add_digit_front(front: usize, original: usize) -> usize {
    front * 10 + original
}

// Helper function for addint digit to back
pub fn add_digit_back(original: usize, back: usize) -> usize {
    original * 10 + back
}

// Then, for each 1-digit fraction, generate all "expanded" fractions.
// and filter for those that are equal to original 1-digit fraction
pub fn find_curious_fractions(a: usize, b: usize) -> Vec<(usize,usize)> {
    let mut res = vec![];

    // generate with expanded digits in:

    // first position
    for i in 1..10 {
        let exp_numerator = add_digit_front(i, a);
        let exp_denominator = add_digit_front(i, b);

        if a as f64 / b as f64 == exp_numerator as f64 / exp_denominator as f64 {
            res.push((exp_numerator, exp_denominator));
        }
    }

    // diag position
    for i in 1..10 {
        let exp_numerator = add_digit_front(i, a);
        let exp_denominator = add_digit_back(b, i);

        if a as f64 / b as f64 == exp_numerator as f64 / exp_denominator as f64 {
            res.push((exp_numerator, exp_denominator));
        }
    }

    // other diag position
    for i in 1..10 {
        let exp_numerator = add_digit_back(a, i);
        let exp_denominator = add_digit_front(i, b);

        if a as f64 / b as f64 == exp_numerator as f64 / exp_denominator as f64 {
            res.push((exp_numerator, exp_denominator));
        }
    }

    // last position
    for i in 1..10 {
        let exp_numerator = add_digit_back(a, i);
        let exp_denominator = add_digit_back(b, i);

        if a as f64 / b as f64 == exp_numerator as f64 / exp_denominator as f64 {
            res.push((exp_numerator, exp_denominator));
        }
    }

    res

}

pub fn reduce_fraction(a: usize, b: usize) -> (usize, usize) {
    //greatest common multiple
    let mut gcm = 1;

    for i in 1..a+1 {
        if a % i == 0 || b % i == 0 {
            gcm = i;
        }
    }

    (a/gcm, b/gcm)
}

fn main() {
    let mut res = vec![];
    // Generate 1-digit fractions
    let one_digit_fractions = generate_one_digit_fractions();

    // for each 1-digit fraction, generate expanded and compare
    for (a,b) in one_digit_fractions {
        res.extend(find_curious_fractions(a,b));
    }

    println!("fractions: {:?}", res);

    // find value of denominator when product of res is in lowest common terms.
    let (numerator, denominator) = res.iter().fold((1,1), |(accum_a, accum_b), &(a, b)| {
        (accum_a * a, accum_b *b)
    });

    println!("res fraction is: {}/{}", numerator, denominator);

    // reduce fraction
    let (_, res) = reduce_fraction(numerator, denominator);

    println!("denominator result is: {}", res);
}
