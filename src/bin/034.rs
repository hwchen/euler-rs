// digit factorials

// numbers which are the same as the sum of digit factorials
//
// Limit: where do number and digit factorials diverge. It happens
// when 9! is involved. so test:
//
// 9! = 362880
// 2 * 9! = 725760 is more than 99
// 3 * 9! = 1088640 is more than 999
// 4 * 9! = 1451520 is more than 9999
// 5 * 9! = 1814400 is more than 99999
// 6 * 9! = 2177280 is more than 999999
// 7 * 9! = 2540160 is less than 9999999, so at this point digit factorials can
//   no longer reach the actual number.
//
// rust is .35s, why is hask 32s?

extern crate euler;

use euler::{to_digits, fac};

fn is_digit_factorial(n: usize) -> bool {
    let factorial_sum: usize = to_digits(n).iter()
        .map(|&x| fac(x))
        .sum();

    factorial_sum == n
}

fn main() {
    let mut res = 0;

    for i in 10..1000000 {
        if is_digit_factorial(i) {
            res += i;
        }
    }

    println!("sum of digit factorials: {}", res);
}
