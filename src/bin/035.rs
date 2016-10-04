// circular prime
//
// 0.31s in rust, 11s in hask. Ah, I think I don't do the easy filter
// of checking digits first in haskell.

extern crate euler;

use euler::{is_prime, to_digits};

fn is_circular_prime(n: usize) -> bool {
    // to check if circular prime:
    // - check if n is prime
    // - convert to digit list
    // - check that all digits are one of: 1, 3, 7, 9
    // - then do cycling (the most expensive part

    if n <= 9 {
        return is_prime(n);

    } else {

        let mut digits = to_digits(n);

        for i in digits.iter() {
            if !(*i == 1 || *i == 3 || *i == 7 || *i == 9) {
                return false;
            }
        }

        if !is_prime(n) { return false; }


        // rotate (n digits) - 1 and test for prime
        for _ in 0..digits.len() {
            rotate_digits(&mut digits);

            //convert to number
            let rotated = digits.clone().into_iter()
                .map(|x| std::char::from_digit(x as u32, 10).unwrap())
                .collect::<String>()
                .parse::<usize>().unwrap();

            if !is_prime(rotated) { return false; }
        }
    }

    true
}

fn rotate_digits(digits: &mut Vec<usize>) -> () {
    // it's a small array, so just pop and insert.
    if let Some(digit) = digits.pop() {
        digits.insert(0, digit);
    }
}

fn main() {
    let mut count = 0;

    for i in 1..1000000 {
        if is_circular_prime(i) {
            count += 1;
        }
    }

    println!("Number of circular primes below 1 million: {}", count);
}
