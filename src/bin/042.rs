// triangle word
//
// let's assum that the highest value possible for 
// a word is z (26) times 20. So 520 is limit.
//
// rust is 0s, hask is 0.01

use std::fs::File;
use std::io::Read;

fn triangle_numbers(limit: usize) -> Vec<usize> {
    (1..).map(|x| x * (x+1) / 2)
        .take_while(|&x| x <= limit)
        .collect::<Vec<_>>()
}



fn main() {
    let tri_nums = triangle_numbers(520);

    let mut f = File::open("p042_words.txt").unwrap();

    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let count_tri_words: usize = buf.split(',')
        .map(|word| {
            // capital A is 65.
            word.trim_matches('\"')
                .chars()
                .map(|c| c as u8 - 64)
                .sum::<u8>() as usize
        }).filter(|n| {
            tri_nums.contains(n)
        }).count();


    println!("num tri words: {}", count_tri_words);
}
