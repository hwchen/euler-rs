#![feature(conservative_impl_trait)]
// pentagonal numbers.
//
// two lists of pentagonal numbers zipped with one shifted by one.
//
// The differences in a pair should only get bigger as  the pentagonal
// numbers get bigger, so the first pair that has a pentagonal
//
// First try seemed similar to Haskell, but was much slower (.67s to 5s)
//
// It's either because of the usage of iterators or the creating of the 
// HashSet, which can't be done lazily.
//
// Well, looks like generating hashset alone takes about 1.66s
// using with_capacity instead of new lowers to 0.75
//
// umm... looks like part of the issue was using a different limit than haskell.
// I was using 10 million here, and 1 million in haskell. So now, runtime
// is down to about 0.15s
//
// working the limits better will give me even lower. But the ballpark is here.

use std::collections::HashSet;

fn gen_pentagonals() -> impl Iterator<Item=usize> {
    (1..).map(|x| x * (3 * x - 1) / 2)
}

fn gen_pentagonals_offset(offset: usize) -> impl Iterator<Item=usize> {
    (1 + offset..).map(|x| x * (3 * x - 1) / 2)
}

fn main() {
    let limit = 10000000;

    let mut pentagonals_set = HashSet::new();

    for n in gen_pentagonals().take_while(|&x| x < limit) {
        pentagonals_set.insert(n);
    }

    let mut res = 0;

    for offset in 1..10000 {
        //println!("new offset {}", offset);
        let pentagonal_pairs_offset = gen_pentagonals()
            .zip(gen_pentagonals_offset(offset))
            .take_while(|&(_,x)| x < 10000000);

        for (a, b) in pentagonal_pairs_offset  {
            let d = b-a;
//            println!("{}", d);
            if pentagonals_set.contains(&(a+b)) && pentagonals_set.contains(&d) {
            //    println!("{}", d);
                res = d;
                break;
            }
        }
    }

    println!("pentagonal pair D: {}", res);
}
