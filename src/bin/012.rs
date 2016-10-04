// Triangle numbers
//
// What's the most efficient way to generate for purpose of testing for first
// with over 500 divisors?

// let's make an iterator. with macro?
//
//
// Rust is 0.49s run time, haskell is 13.64s
//
// I think I even used more tricks in haskell?

#[macro_use]
extern crate euler;

use euler::factors;

fn main() {
    let mut triangle = recurrence!(a[n]: usize = 0, 1 ;; a[n-1] + n);

    let solution;

    loop {
        let i = triangle.next().unwrap();
        if factors(i).len() > 500 {
            solution = i;
            break;
        }
    }

    println!("First triangle number to have > 500 divisors: {}", solution);
}
