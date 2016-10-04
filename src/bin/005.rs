extern crate euler;

use euler::is_prime;

fn is_evenly_div_by_range(n: usize, start: usize, end: usize) -> bool {
    for i in start..end+1 {
        if n % i != 0 {
            return false
        }
    }
    true
}

fn main() {

    let n = 20;
    let mut smallest_multiple = 0;

    // get all prime factors and find product.
    // Any candidates must be a product of this
    // prime factors product
    let multiplier = (1..n+1)
        .filter(|&i| {
            is_prime(i)
        }).fold(1, |prod, x| {
            prod * x
        });
    println!("{}", multiplier);


    // generate candidates in a lazy iterator
    // Not done in separate named function because
    // it's easier to pass the iterator this way.
    // if doing it through function, cleanest way
    // to do would be constucting a vec, but then
    // you need to know when to stop. Here, can
    // just create an infinite iterator and stop
    // the for loop when necessary.
    let candidates = (1..)
        .map(|i| {
            i * multiplier
        });

    for n in candidates {
        if is_evenly_div_by_range(n, 2, 20) {
                smallest_multiple = n;
                break;
            }
    }
    println!("Smallest pos num evenly div by all num from 1-20 {}", smallest_multiple);
}
