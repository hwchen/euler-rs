// Find max sum path from top to bottom of triangle

use std::cmp;

fn main() {
    let mut triangle: Vec<Vec<usize>> = INPUT.lines()
        .map(|line| {
            line
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect()
        }).collect();

    // start with a row and pad one on each to make it easier to add to below
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

const INPUT: &str =
"75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

