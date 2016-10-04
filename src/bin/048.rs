// last 10 digits of series 1^1... 1000^1000
//
// 13s for my bignum implementation
// .01s for haskell. Pretty sure they have better bignum than me,
// and better multiplication and stuff.

extern crate euler;

use euler::bignum::BigNum;

fn main() {
    let sum = (1..1001).map(|x| {
        let n: BigNum = x.to_string().parse().unwrap();
        n.pow(x)
    }).fold("0".parse::<BigNum>().unwrap(), |sum, x| sum + x);

    println!("{}", sum);
}
