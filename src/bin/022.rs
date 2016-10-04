use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn name_score(name: &str, scoresheet: &HashMap<char, u64>) -> u64 {

    name.chars()
        .map(|c| scoresheet.get(&c.to_lowercase().next().unwrap()).unwrap())
        .fold(0, |sum, &x| sum + x)
}

fn main() {
    // alpha score sheet
    // better way? in python would use dict
    let mut scoresheet = HashMap::new();
    for (alpha, i) in "abcdefghijklmnopqrstuvwxyz".chars().zip(1..) {
        scoresheet.insert(alpha, i);
    }

    let mut f = File::open("p022_names.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut name_list: Vec<&str> = input.split(",")
        .map(|name| name.trim_matches('\"'))
        .collect();

    name_list.sort();

    let sum = name_list.iter().zip(1..)
        .map(|(name, i)| i * name_score(name, &scoresheet))
        .fold(0, |sum, x| sum + x);

    println!("sum of name scores: {}", sum);
}
