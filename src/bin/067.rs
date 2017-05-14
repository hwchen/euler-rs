// Find max sum path from top to bottom of triangle
// Same algo as 018

use std::cmp;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("p067_triangle.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut triangle: Vec<Vec<usize>> = buf.lines()
        .map(|line| {
            line
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect()
        }).collect();

    // start with a row and pad zero on each to make it easier to add to below
    // for the row below, take the processed above row and add from the window of
    // two above, taking the max. Then slide down.

    let k = triangle.len();
    for (i,j) in (0..k-1).zip(1..k) {
        triangle[i].push(0);
        triangle[i].insert(0,0);

        let parent_row = triangle[i].clone();
        let parent_pairs = parent_row.windows(2);
        for (m, parent_pair) in triangle[j].iter_mut().zip(parent_pairs) {
            *m = cmp::max(*m + parent_pair[0], *m + parent_pair[1]);
        }
    }

    // now take the max of the last row (which should be all sums)
    println!("Max path sum: {:?}", triangle.pop().unwrap().iter().max().unwrap());
}

