#![feature(conservative_impl_trait)]
// pandigital number.
//
// Formed from concatenated product of an integer m with 1...n, n > 1
//
// First, what is the limit of i and limit of n?
//
// There can only be 9 digits total. The bigger the m, the smaller n is.
//
// When m is 5, n is 1 or less // doesn't count, n must be > 1
// When m is 4, n is 2 or less
// When m is 3, n is 3 or less (can't overshoot # of digits)
// When m is 2, n is 4 or less
// when m is 1, n is 9 or less
//
// so, i is btwn 1..4, n is between 2 and 9
// 
// I'm doing something inefficient here! hask is finally faster
// than rust, .02 to .05. I wonder if there's a cool optimization
// from hask compiler.

extern crate euler;

use euler::{is_pandigital, to_digits};

fn test_ranges(n: usize) -> impl Iterator<Item= impl Iterator<Item=usize>> {
    (2..n+1).map(|x| (1..x+1))
}

// oops, no guarantee that it's only single digits in vec.
fn vec_to_int(v: &Vec<usize>) -> usize {
    v.iter()
        .map(|&x| std::char::from_digit(x as u32, 10).unwrap())
        .collect::<String>()
        .parse().unwrap()
}

fn main() {
    let mut largest_pandigital = 0;

    // m must not have repeating elements.

    // when m is 1 digit, n is 9 or less
    // because order matter, 123456789 is largest pandigital here.

    // when m is 2 digit, n is 4 or less
    for m in 10..100 {
        for n in 2..4+1 {
            for test_range in test_ranges(n) {
                let test_x_digits: Vec<_> = test_range
                    .flat_map(|i| to_digits(m *i))
                    .collect();

                let test_x = vec_to_int(&test_x_digits);

                if is_pandigital(test_x_digits) && test_x > largest_pandigital {
                    largest_pandigital = test_x;
                }
            }
        }
    }
    // when i is 3 digit, n is 3 or less
    for m in 100..1000 {
        for n in 2..3+1 {
            for test_range in test_ranges(n) {
                let test_x_digits: Vec<_> = test_range
                    .flat_map(|i| to_digits(m *i))
                    .collect();

                let test_x = vec_to_int(&test_x_digits);

                if is_pandigital(test_x_digits) && test_x > largest_pandigital {
                    largest_pandigital = test_x;
                }
            }
        }
    }
    // when i is 4 digit, n is 2 or less
    for m in 1000..10000 {
        for n in 2..2+1 {
            for test_range in test_ranges(n) {
                let test_x_digits: Vec<_> = test_range
                    .flat_map(|i| to_digits(m *i))
                    .collect();

                let test_x = vec_to_int(&test_x_digits);

                if is_pandigital(test_x_digits) && test_x > largest_pandigital {
                    largest_pandigital = test_x;
                }
            }
        }
    }

    println!("Largest pandigital from products: {}", largest_pandigital);
}
