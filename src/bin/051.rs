// replacing parts of number with same digit, eight prime value family

// must be at least 4 digits long
//
// how many primes are there at each digit length?
//
// I think best technique is still to group primes together
//
// Ah, the replacement can never be the last digit. So must be
// at least 5 digits long.
//
// rust is 2.8s, hask is 10.1s..
// And wow, I even limited the number of replacement spots in hask
// to 3. But replacing a spot in haskell is only done one spot at
// a time, that must be slow.

extern crate euler;
extern crate itertools;

use euler::{is_prime, to_digits, digits_to_int};
use itertools::Itertools;

// returns a vec of vec of numbers, with digits replace from 0-9
// at the places specified
fn replace_digits_family(digits: &[usize], places: &[usize]) -> Vec<Vec<usize>> {
    let mut res = vec![];

    for n in 0..10 {
        let mut temp_digits = digits.clone().to_vec();
        for index in places.iter() {
            temp_digits[*index] = n;
        }
        res.push(temp_digits);
    }

    res.to_vec()
}

fn places_to_replace(digits_length: usize) -> Vec<Vec<usize>> {
    let mut res = vec![];

    for n in 1..digits_length {
        res.extend((0..digits_length).combinations(n).collect::<Vec<Vec<usize>>>());
    }

    res
}

fn is_prime_family_eight(p: usize, places_replace_combinations: &Vec<Vec<usize>>) -> bool {
    let digits = to_digits(p);

    // currently generating places each time, but could be lazy_static?
    for places in places_replace_combinations.iter() {
        let family = replace_digits_family(&digits, &places);
        let family_primes = family.iter()
            .map(|digits| digits_to_int(digits))
            .filter(|&x| is_prime(x))
            .collect::<Vec<_>>();

        if family_primes.len() == 8 && family_primes[0] == p {
            return true;
        }
    }
    false
}

fn main() {
    let five_primes = (10000..100000).filter(|&x| is_prime(x));

    let places_for_five_digits = places_to_replace(5);
    for p in five_primes {
        if is_prime_family_eight(p, &places_for_five_digits) {
            println!("family of 8 primes w/ smallest prime: {}", p);
            break;
        }
    }

    let six_primes = (100000..1000000).filter(|&x| is_prime(x));

    let places_for_six_digits = places_to_replace(6);
    for p in six_primes {
        if is_prime_family_eight(p, &places_for_six_digits) {
            println!("family of 8 primes w/ smallest prime: {}", p);
            break;
        }
    }
}
