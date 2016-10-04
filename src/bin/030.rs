// numbers that can be written as 5th power of digits.
//
// Question is how many digits to search.
//
// As number gets bigger, its digits also have to become bigger
//
// let's just brute force 5 digits first
//
// Upper bound:
//
// n9^5, `when n=6, sum = 6 digits. when n = 7, sum still = 6 digits.
//
// That means that at 7 digits, it's not possible to reach the maximum
// sum anymore.
//
// So far found numbers up to 6 digits!

// .36s in rust, over 1 min 24s in hask

extern crate euler;

use euler::to_digits;

fn list_num_sum_fifth_pow() -> Vec<usize> {
    let mut res = vec![];
    for i in 2usize..1000000 {
        let sum_pow_digits = to_digits(i).iter().map(|x| x.pow(5)).sum();
        if i == sum_pow_digits {
            res.push(i);
        }
    }
    res
}

fn main() {
    println!("sum: {}", list_num_sum_fifth_pow().iter().sum::<usize>());
}
