use std::collections::HashMap;
use std::io;

fn main() {
    let mut lefts: Vec<u32> = Vec::new();
    let mut right_freqs: HashMap<u32, u32> = HashMap::new();
    for line in io::stdin().lines().filter_map(Result::ok) {
        let mut parts = line.split_whitespace();
        lefts.push(parts.next().expect("no left value").parse().unwrap());
        let right = parts.next().expect("no right value").parse().unwrap();
        let right_entry = right_freqs.entry(right).or_insert(0);
        *right_entry += 1
    }
    let mut score: u32 = 0;
    for left in lefts {
        score += left * right_freqs.get(&left).unwrap_or(&0)
    }
    println!("{}", score);
}
