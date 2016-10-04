// distinct values of nCr for 1 <= n <= 100, how many
// are greater than 1 million.
//
//
// TODO Ugh, need to implement bignum subtract, divide, and Ord for this.
// I need to clean it up for real, so I can understand it.

extern crate euler;
extern crate itertools;

use euler::bignum::BigNum;
use euler::to_digits;
use itertools::Itertools;

// nCr
fn choose(n: usize, r: usize) -> usize {
    // assumes that n larger than r, so don't check

    let diff = (n-r).to_string().parse::<BigNum>().unwrap();
    let n = n.to_string().parse::<BigNum>().unwrap();
    let r = r.to_string().parse::<BigNum>().unwrap();
    n.fac() / (r.fac() * (diff).fac())
}

fn main() {
    println!("{}", choose(23, 10));
}
