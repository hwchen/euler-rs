extern crate euler;

use euler::is_prime;

fn testing_primes() {
    let mut soln = 0;
    let mut count = 0;

    for i in 2.. {
        if is_prime(i) {
            count += 1;
        }
        if count == 10001 {
            soln = i;
            break;
        }
    }

    println!("10,001st prime is: {}", soln);
}

//fn generating_primes() {
// hard to use, because sieves generally generate primes up
// to a limit, not the first n primes.
//}

fn main() {
    testing_primes();
}
