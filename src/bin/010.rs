// very similar code to haskell, but haskell is 16.5 s and
// rust is 1.5 seconds. But would have to look more
// closely at haskell code to know for sure.
extern crate euler;

use euler::is_prime;

fn main() {
    let limit = 2000000;
    let sum_of_primes = (2..limit+1)
        .filter(|&i| is_prime(i))
        .fold(0, |sum, i| sum + i);

    println!("Sum of primes below {}: {}", limit, sum_of_primes);
}
