// need to make fib generator using my bignum representation
// agh, too much pain to switch the recurrence macro right now.
//
// I'll just generate a ton of fibs and make an iterator from there.

extern crate euler;

use euler::bignum::BigNum;
//use euler::fib_bignum_seq_limit_index;

fn main() {
//    let mut fibs = fib_bignum_seq_limit_index(4783).into_iter().zip(1..)
    let mut fibs = FibBigNum::new().zip(1..)
        .skip_while(|&(ref n,_)| {
            n.digit_len() < 1000
        });
    let (_, i) = fibs.next().unwrap();
    println!("Fib using bignum, first over 1000 digits: index {}", i);
}

// I want to make this an iterator, how do I fix the recurrence macro?
// Even generating the vec first and then iterating over it, it's
// still faster than haskell!
//
//
// It ended up from .03-.04 seconds to .02 seconds using just iterator!
// And that's even with tons of clones.

struct FibBigNum {
    n: BigNum,
    n_minus_one: BigNum,
    n_minus_two: BigNum,
}

impl FibBigNum {
    pub fn new() -> Self {
        FibBigNum {
            n: "1".parse::<BigNum>().unwrap(),
            n_minus_one: "0".parse::<BigNum>().unwrap(),
            n_minus_two: "0".parse::<BigNum>().unwrap(),
        }
    }
}

impl Iterator for FibBigNum {
    type Item = BigNum;

    fn next(&mut self) -> Option<BigNum> {
        let res = self.n.clone();
        self.n_minus_two = self.n_minus_one.clone();
        self.n_minus_one = self.n.clone();
        self.n = self.n_minus_two.clone() + self.n_minus_one.clone();

        Some(res)
    }
}
