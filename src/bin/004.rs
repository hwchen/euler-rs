extern crate euler;

use euler::is_palindrome;

fn main() {
    let mut max_palindrome = 0;
    // how to make ordered cartesian product (lazy?)
    for j in 1..1000 {
        for k in 1..1000 {
            let n = j * k;
            let s = format!("{}", n);
            if is_palindrome(&s) {
                if n > max_palindrome {
                    max_palindrome = n;
                }
            }
        }
    }

    println!("Largest palindrome product of two 3-digit numbers is: {}", max_palindrome);
}
