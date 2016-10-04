#![feature(conservative_impl_trait)]
// longest string of consecutive primes that sums to a prime.
//
// For prime below 100, below 1000, below 1000000.
//
// a sequence should be tested until its sum is either equal to
// or over the target. Starting from 2 and going to starting
// from p/2
// And have to do all of them and find the max length
//
// Ugh, on first pass, hask is 4s and rust is 11.75s
// It's even slower when returning &[] instead of vec? crazy!
//
//
// I'm going to profile!

extern crate euler;

use euler::is_prime;

fn gen_primes(limit: usize) -> impl Iterator<Item=usize> {
    (1..limit).filter(|&x| is_prime(x))
}

fn sum_consec_seq(target: usize, master_seq: &[usize]) -> Option<Vec<usize>> {
    // push candidate sequences here, and find longest
    // wait, first sequence must be longest, but it contains
    // the smallest numbers

    // referencing array indices is simpler than using iterator
    // but maybe it's slower than generating iterators?
    for start in 0..master_seq.len()/2 {
        let mut prime_seq_running_total = 0;
        let mut i = start;
        while prime_seq_running_total < target {
            prime_seq_running_total += master_seq[i];
            i += 1;
        }
        if prime_seq_running_total == target {
            //println!("{}, {}, {}", start, i, target);
            return Some(master_seq[start..i].to_vec());
        }
    }
    None
}

fn longest_seq(master_seq: &[usize]) -> Vec<usize> {
    master_seq.iter().filter_map(|&x| sum_consec_seq(x, master_seq))
        .max_by_key(|seq| seq.len()).unwrap()
}

fn main() {
    let limit = 1000000;

    let primes: Vec<_> = gen_primes(limit).collect();

    println!("{:?}", longest_seq(&primes).iter().sum::<usize>());
}
