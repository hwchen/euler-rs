// double-base palindromes
//  0.17s, 0.34 for hask

extern crate euler;

use euler::is_palindrome;

fn decimal_to_binary(n: usize) -> String {
    let mut n = n;

    let mut res = vec![];

    if n == 0 { res.push(0); }

    while n > 0 {
        res.push(n % 2);
        n = n/2;
    }

    res.into_iter()
        .rev()
        .map(|i| std::char::from_digit(i as u32, 2).unwrap())
        .collect::<String>()
}


fn main() {
    let mut sum = 0;

    for i in 0..1000000 {
        let s = format!("{}", i);
        if is_palindrome(&s) && is_palindrome(&decimal_to_binary(i)) {
            sum += i;
        }
    }

    println!("Sum of numbers that are base 2 and 10 palindromes below one million: {}", sum);
}
