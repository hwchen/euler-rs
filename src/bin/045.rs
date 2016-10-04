// hexagonal and pentagonal and triangle number

// the idea is to create sets to check against on the fly
// triangle number has the highest n, pentagonal and hexagonal
// numbers will have lower n for the same number. So create
// sets to check against for P and H
//
// made mistake first time through of only iterating over
// triangle numbers, instead of all n
//
// Much faster for rust: 0.26s versus 25.59
// ah, I only used lists in haskell, no sets.

use std::collections::HashSet;
use std::usize::MAX;

fn main() {
    let mut p = HashSet::new();
    let mut h = HashSet::new();

    let mut res = 0;

    for n in 144.. {
        let t_num = n*(n+1)/2;
        let p_num = n*(3*n-1)/2;
        let h_num = n*(2*n-1);

        println!("{}, {}, {}", t_num, p_num, h_num);

        // check t num because it takes longest to get to.
        if p.contains(&t_num) && h.contains(&t_num) {
            res = n;
            break;
        }
        // insert Hn and Pn here
        p.insert(p_num);
        if h_num < MAX { // to avoid errors from h overflowing.
            h.insert(h_num);
        }
    }

    println!("t, p, h number: {}", res * (res + 1) / 2);
}
