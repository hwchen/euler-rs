// truncatable primes
//
// find only primes that can be truncatable from both left and right
//
// 2,3,5,7 are not considered truncatable
//
// Only 1,3,7,9 can be digits. Because in truncation, all other digits
// will at some point be the last one.
//
// Now the question is what the upper bound is. (there is none, they
// tell us that it's only 11 numbers.
//
// hask time is 3.6s, rust is 0.28. And hask also just takes 11. wow,
// big difference.
// 
extern crate euler;

use euler::{is_prime, to_digits};

fn is_truncatable_prime(n: usize) -> bool {
    if n < 10 {
        return false;
    } else {
        let mut digits = to_digits(n);

        // first check digits
        // oops, I thought it was like circular primes.
        // but because it's ok that a single digit is checked,
        // 2 and 5 can also be possible digits.
        // time went from .68 to .27 just by checking digits from no checking.
        for i in digits.iter() {
            if !(*i == 1 || *i ==2 || *i == 3 || *i == 5 || *i == 7 || *i == 9) {
                return false;
            }
        }

        // then check if prime
        if !is_prime(n) {
            return false;
        }

        // check truncating from left
        // first time, uses a shadowed digits
        {
            let mut digits = digits.clone();
            for _ in 0..digits.len()-1 {
                digits.remove(0);

                let truncated_n = digits.iter()
                    .map(|&i| std::char::from_digit(i as u32, 10).unwrap())
                    .collect::<String>()
                    .parse::<usize>().unwrap();

                if !is_prime(truncated_n) {
                    return false;
                }
            }
        }

        // check truncating from right
        // this one uses original digits
        for _ in 0..digits.len()-1 {
            digits.pop();

            let truncated_n = digits.iter()
                .map(|&i| std::char::from_digit(i as u32, 10).unwrap())
                .collect::<String>()
                .parse::<usize>().unwrap();

            if !is_prime(truncated_n) {
                return false;
            }
        }
    }

    true
}

fn main() {

    let mut trunc_primes = vec![];

    let mut i = 1;

    while trunc_primes.len() < 11 {
        if is_truncatable_prime(i) {
            trunc_primes.push(i)
        }
        i += 1
    }

    let sum: usize = trunc_primes.iter().sum();
    println!("sum of trunc primes: {}", sum);
}
