// integer right triangles
//
// a^2 + b^2 = c^2
//
// Need to generate triplets for which a+b+c = p.
//
// p is anywhere from 1 to 1000. which p has the most triplets?
//
// rust is 0.1, hask is 0.8. But this might be attributable to
// hask picking a, b, and c while rust only picks a and c.
// However, also possible that hask optimizes away that picking.
// Rust did not, also choosing b made this program very slow.
//
// making a go to p/4 instead of p/2 cut down time to 0.04

use std::collections::HashSet;

fn generate_triplets(p: usize) -> HashSet<(usize, usize, usize)> {
    let mut res = HashSet::new();
    for a in 1..p/4 {
        for c in 1..p/2 {
            if c + a < p {
                let b = p - (c + a);
                if a.pow(2) + b.pow(2) == c.pow(2) {
                    res.insert((a, b, c));
                }
            }
        }
    }
    res
}

fn main() {
    let mut max_solutions = 0;
    let mut max_solutions_at = 0;
    // generating triplets for p
    // c can be at most half of p? is my guess.
    for p in 1..1000 {
        let triplets = generate_triplets(p);

        //println!("{:?}", triplets);
        let num_solutions = triplets.len();

        if num_solutions > max_solutions {
            max_solutions = num_solutions;
            max_solutions_at = p;
        }
    }

    println!("Max solutions for p at: {}", max_solutions_at);
}
