// Reciprocal cycles
//
// logic bugs and notes:
//
// - no remainder means no cycle
// - cycle doesn't always start on tenths place
//
// Why so slow? it's 0.30 compared to haskell 0.47.
//
// I don't have extra allocations, because I'm taking by_ref for .cycle_len()
// Where else is slow?
//
// Ah, I'm also testing stuff that's not prime.
// Now it's down to 0.22. But hask version was not efficient anyways.
//
// Looks like there's a trick of testing: multiply by 10 and divide by d until rem 1.

extern crate euler;

use euler::is_prime;

// decimal derived from unit fraction (1/n)
#[derive(Debug)]
struct UnitDecimal {
    denominator: usize,
    last_dividend_digit: usize,
}

impl UnitDecimal {
    pub fn new(denominator: usize) -> Self {
        UnitDecimal {
            denominator: denominator,
            last_dividend_digit: 1,
        }
    }

    pub fn cycle_len(&mut self) -> usize {
        let mut test_cycle_len = 0;

        loop {
            test_cycle_len += 1;

            let test_cycles: Vec<_> = self.by_ref().take(test_cycle_len * 2).collect();
            let (test1, test2) = test_cycles.split_at(test_cycle_len);

            if test1 == test2 {
                break;
            }
        }

        test_cycle_len
    }
}

impl Iterator for UnitDecimal {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let pad_right_places = (self.denominator as f64).log(10f64).floor() as u32 + 1;
        let d = self.last_dividend_digit * 10usize.pow(pad_right_places);
        let out = d / self.denominator;
        self.last_dividend_digit = d % self.denominator;

        Some(out)
    }
}

// Should only test prime d
fn longest_cycle_limit(limit: usize) -> (usize, usize) {
    let mut longest_cycle = 0;
    let mut d = 0;

    for i in (1..limit).filter(|&x| is_prime(x)) {
        let cycle_len = UnitDecimal::new(i).cycle_len();
        if cycle_len > longest_cycle {
            d = i;
            longest_cycle = cycle_len;
        }
    }
    (d, longest_cycle)
}

fn main() {
    let (d, longest_cycle) = longest_cycle_limit(1000);
    println!("Longest cycle for d < 1000: {} with cycle {}", d, longest_cycle);
}
