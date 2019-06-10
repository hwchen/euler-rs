// one-millionth lexicographic permutation of digits 0-9.
// for first round, 0 is always in front. So the permutations
// for 1-9, or the first 9! = 362880.
// with 1 in front, 362880 * 2 = 725760
// so I can just do permutations with 2 in front, 0,1,3,4,5,6,7,8,9.
// and find out the 1,000,000 - 725760 permutation of that set.
//
// 1234
// 1243
// 1324
// 1342
// 1423
// 1432
// 2134

fn lex_permute(set: &mut[u8], n: usize) -> usize {
    assert!(n > 0);
    let last_index = set.len() - 1;
    let mut current_index = last_index;

    for _ in 1..n {
        println!("{:?}", set);
        // swap element
        if current_index > 0 {
            set.swap(current_index, current_index -1);
            current_index = current_index - 1;
        } else {
            set.swap(current_index, last_index);
            current_index = last_index;
        }
    }
    println!("The {}th time: {:?}", n, set);

    set.iter()
        .map(|&i| i.to_string())
        .fold(String::new(), |mut acc, c| {
              acc.push_str(&c);
              acc
        })
        .parse()
        .unwrap()
}

fn fac(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        n => (1..n+1).fold(1, |acc, n| n * acc),
    }
}

fn main() {
    let mut set: [u8;9] = [0,1,3,4,5,6,7,8,9];
    println!("{}", lex_permute(&mut set, 3));
}
