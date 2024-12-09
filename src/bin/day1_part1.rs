use std::io;

fn main() {
    let mut lefts: Vec<u32> = Vec::new();
    let mut rights: Vec<u32> = Vec::new();
    for line in io::stdin().lines().filter_map(Result::ok) {
        let mut parts = line.split_whitespace();
        lefts.push(parts.next().expect("no left value").parse().unwrap());
        rights.push(parts.next().expect("no right value").parse().unwrap());
    }
    lefts.sort();
    rights.sort();
    let mut diff: u32 = 0;
    for (left, right) in lefts.iter().zip(rights) {
        diff += left.abs_diff(right)
    }
    println!("{}", diff);
}
