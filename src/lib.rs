#![feature(conservative_impl_trait)]
// TODO switch all to usize!)
extern crate owned_chars;

pub mod bignum;
mod recurrence;

use bignum::BigNum;
use owned_chars::OwnedCharsExt;
use std::collections::HashMap;


/// returns a fib sequence n terms long starting with [1,2,3,5,8...]
pub fn fib_seq(n: usize) -> Vec<usize> {
    match n {
        0 => vec![],
        1 => vec![1],
        2 => vec![1,1],
        3 => vec![1,1,2],
        _ => {
            let mut cache = HashMap::new();
            cache.insert(1, 1);
            cache.insert(2, 2);


            for i in 4..n+1 {
                // Can I do it without these clones? although, shouldn't be
                // too expensive since they're u64.
                let a = cache.get(&(i-2)).ok_or("bug in cache get").unwrap().clone();
                let b = cache.get(&(i-1)).ok_or("bug in cache get").unwrap().clone();
                cache.insert(i, a + b);
            }

            cache.values().cloned().collect::<Vec<_>>()
        },
    }
}

/// returns a fib sequence as long as n < limit, starting with [1,2,3,5,8...]
pub fn fib_seq_limit(limit: usize) -> Vec<usize> {
    match limit {
        0 => vec![],
        1 => vec![1],
        2 => vec![1,1],
        3 => vec![1,1,2],
        _ => {
            let mut cache = HashMap::new();
            cache.insert(1, 1);
            cache.insert(2, 1);
            cache.insert(3, 2);

            let mut i = 4;
            loop {
                // Can I do it without these clones? although, shouldn't be
                // too expensive since they're u64.
                let a = cache.get(&(i-2)).ok_or("bug in cache get").unwrap().clone();
                let b = cache.get(&(i-1)).ok_or("bug in cache get").unwrap().clone();
                let c = a + b;
                if c < limit {
                    cache.insert(i, a + b); // shouldn't a+b just be c?
                    i += 1;
                } else {
                    break;
                }
            }

            cache.values().cloned().collect::<Vec<_>>()
        },
    }
}

/// returns a bignum fib sequence as long as n < limit, starting with [1,2,3,5,8...]
pub fn fib_bignum_seq_limit_index(limit: usize) -> Vec<BigNum> {
    // because I need to have in order for this problem, need to
    // push onto vec.
    match limit {
        0 => vec![],
        1 => vec!["1".parse::<BigNum>().unwrap()],
        2 => vec!["1".parse::<BigNum>().unwrap(),
                  "1".parse::<BigNum>().unwrap()],
        3 => vec!["1".parse::<BigNum>().unwrap(),
                  "1".parse::<BigNum>().unwrap(),
                  "2".parse::<BigNum>().unwrap()],
        _ => {
            let mut res = vec!["1".parse::<BigNum>().unwrap(),
                           "1".parse::<BigNum>().unwrap(),
                           "2".parse::<BigNum>().unwrap()];

            let mut cache = HashMap::new();
            cache.insert(1, "1".parse::<BigNum>().unwrap());
            cache.insert(2, "1".parse::<BigNum>().unwrap());
            cache.insert(3, "2".parse::<BigNum>().unwrap());

            let mut i = 4;
            loop {
                // Can I do it without these clones? although, shouldn't be
                // too expensive since they're u64.
                if i < limit {
                    let a = cache.get(&(i-2)).ok_or("bug in cache get").unwrap().clone();
                    let b = cache.get(&(i-1)).ok_or("bug in cache get").unwrap().clone();
                    let c = a + b;
                    cache.insert(i, c.clone());
                    i += 1;
                    res.push(c);
                } else {
                    break;
                }
            }

            res
        },
    }
}

pub fn is_prime(n: usize) -> bool {
    let limit = (n as f64).sqrt() as usize;

    if n == 1 || n == 0 { return false; }

    for i in 2..limit+1 {
        if n % i == 0 {
            return false
        }
    }

    true
}

pub fn factors(n: usize) -> Vec<usize> {
    let mut factors = vec![1];
    let limit = (n as f64).sqrt() as usize;

    for i in 2..limit+1 {
        if n % i == 0 {
            factors.push(i);

            let other_div = n / i;
            if other_div != limit {
                factors.push(other_div);
            }
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

/// factors, but without n
pub fn proper_divisors(n:usize) -> Vec<usize> {
    let mut factors = vec![1];
    let limit = (n as f64).sqrt() as usize;

    for i in 2..limit+1 {
        if n % i == 0 {
            factors.push(i);

            let other_div = n / i;
            if other_div != limit {
                factors.push(other_div);
            }
        }
    }

    factors
}

pub fn is_palindrome(s: &str) -> bool {
    // is string parsing the best way?
    // didn't try to parse, i think better to just turn all
    // to string
    s == s.chars().rev().collect::<String>()
}

pub fn fac(n: usize) -> usize {
    match n {
        0 => 1,
        _ => (1..n+1).fold(1, |prod, x| { prod * x}),
    }
}

//TODO impl Iterator test this on nightly after 9/17.
//pub fn to_digits(n: usize) -> impl Iterator <Item=usize> {
//    n.to_string()
//        .into_chars()
//        .map(|c| c.to_digit(10).unwrap() as usize)
//}

pub fn to_digits(n: usize) -> Vec<usize> {
    n.to_string()
        .into_chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
}

pub fn digits_to_int(digits: &[usize]) -> usize {
    let mut res = 0;

    let digits_len = digits.len();

    for i in 0..digits_len {
        res += 10u32.pow(i as u32) * digits[digits_len -1 - i as usize] as u32;
    }
    res as usize
}

pub fn digits_sum(n:usize) -> usize {
    to_digits(n).into_iter().fold(0, |sum, i| sum + i)
}

// Ended up just using itertools cartesian product. Kind of a pain
// to implement an iterator for it here.

// Can't decide whether to implement with n as vec or as int
pub fn is_pandigital(digits: Vec<usize>) -> bool {
    if digits.len() != 9 {
        return false;
    }
    
    let mut digits = digits;

    digits.sort();

    if digits != vec![1,2,3,4,5,6,7,8,9] {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_test() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
        assert_eq!(is_prime(9), false);
    }

    #[test]
    fn factors_test() {
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(2), vec![1,2]);
        assert_eq!(factors(3), vec![1,3]);
        assert_eq!(factors(4), vec![1,2,4]);
        assert_eq!(factors(10), vec![1,2,5,10]);
        assert_eq!(factors(18), vec![1,2,9,3,6,18]);
    }

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome("1"), true);
        assert_eq!(is_palindrome("101"), true);
        assert_eq!(is_palindrome("1001"), true);
        assert_eq!(is_palindrome("10011"), false);
    }

    #[test]
    fn fac_test() {
        assert_eq!(fac(0), 1);
        assert_eq!(fac(1), 1);
        assert_eq!(fac(2), 2);
        assert_eq!(fac(3), 6);
        assert_eq!(fac(4), 24);
    }

    #[test]
    fn to_digits_test() {
        assert_eq!(to_digits(1), vec![1]);
        assert_eq!(to_digits(11), vec![1,1]);
        assert_eq!(to_digits(111), vec![1,1,1]);
        assert_eq!(to_digits(1111), vec![1,1,1,1]);
        assert_eq!(to_digits(11111), vec![1,1,1,1,1]);
    }

    #[test]
    fn digits_sum_test() {
        assert_eq!(digits_sum(1), 1);
        assert_eq!(digits_sum(11), 2);
        assert_eq!(digits_sum(111), 3);
        assert_eq!(digits_sum(1111), 4);
        assert_eq!(digits_sum(11111), 5);
        }
    }
