// permuated multiples
//
// smallest integer x such than 2x, 3x, 4x, 5x, 6x contain same digits
//
// Strategies: could generate permutations, or just sort and compare.
// Generating permutations creates a lot of test cases.
//
// hask is 0.22s, rust is 0.29. This is pretty simple, so curious
// where the time is taken. This is a good one to profile?
//
// looks like je_arena_ralloc and rallocx take most calls.
// I assume that this is from allocating?
//
// Let's see what happens when I allocate less.
//
// Wow, removing the intermediate step of allocating multiples to
// a Vec and then getting a ref to it and re-iterating over it
// was very expensive! Now only 0.16s.
// Vec allocation is to the heap, I think.

extern crate euler;

use euler::to_digits;

fn is_permuted_multiples(x: usize) -> bool {
    let mut multiples = (2..7).map(|n| {
        let mut digits = to_digits(n * x);
        digits.sort();
        digits
    });

    let comparator = multiples.next().unwrap();
    multiples.all(|n| comparator == n)
}

fn main() {
    for x in 1.. {
        if is_permuted_multiples(x) {
            println!("permuted multiples: {}", x);
            break;
        }
    }
}
