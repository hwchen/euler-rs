//concatenating sequence
//
//sequence concatenating positive integers (can start at 0, with 0 index).
//
// rust took 0.48s, hask was 0.11. And that's even using lists!
//
// So, there must be a faster way for rust. I think that extending the vec
// is relatively costly, especially when to_digits has to continuously
// build a list of chars first. so try better implementation here.
//
// 0.30s in rust now with faster to_digits implementation. But reversing
// is slow!
//
// would flatmap make the full list generation faster?
//

//extern crate euler;

//use euler::to_digits;

fn to_digits(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut res = vec![];

    while n > 0 {
        res.push(n % 10);
        n = n / 10;
    }

    res.reverse();
    res
}

fn concat_int_seq(limit: usize) -> Vec<usize> {
    let mut res = vec![0];

    for i in 1..limit+1 {
        let digits = to_digits(i);
        res.extend(digits);
    }
    res
}

fn main() {
    let d = concat_int_seq(1000000);
    let sum = d[1] * d[10] * d[100] * d[1000] * d[10000] * d[100000] * d[1000000];

    println!("sum of bits of d: {}", sum);
}
