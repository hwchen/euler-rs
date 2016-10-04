// Quadratic primes

// a is isize from -1000 to 1000 not inclusive
// b is isize from -1000 to 1000 inclusive
//
// n^2 + an + b
//
// find product of coefficients that produces:
//
// max number of primes for consecutive values of n starting with n = 0

extern crate euler;

use euler::is_prime;

fn quadratic_seq_prime_len(a: isize, b: isize) -> isize {
    let mut n: isize = 0;

    loop {
        let quadratic_res = n.pow(2) + (a*n) + b;

        if quadratic_res < 0 { break; }

        if is_prime(quadratic_res as usize) {
            n += 1;
        } else {
            break;
        }
    }

    n
}

// returns the coefficients of the quad equation that returns longest
// sequence of primes
fn longest_quadratic_seq_prime_len() -> (isize, isize) {
    let mut max_prime_len = 0;
    let mut res = (0,0);

    for a in -999..1000 {
        for b in -1000..1001 {
            let seq_len = quadratic_seq_prime_len(a, b);
            if seq_len > max_prime_len {
                max_prime_len = seq_len;
                res = (a,b);
            }
        }
    }
    res
}

fn main() {
    let (a,b) = longest_quadratic_seq_prime_len();
    println!("prod of a b: {}", a*b);
//    let res = quadratic_seq_prime_len(-79, 1601);
//    println!("{}", res);
//    let res = quadratic_seq_prime_len(1, 41);
//    println!("{}", res);
}
