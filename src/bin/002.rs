extern crate euler;

use euler::fib_seq_limit;

fn main() {
    let fib_even_sum = fib_seq_limit(4000000).into_iter()
        .filter(|i| {
            i % 2 == 0
        }).fold(0, |sum, i| {
            sum + i
        });

    println!("Sum of even fib below 4,000,000: {}", fib_even_sum);
}
