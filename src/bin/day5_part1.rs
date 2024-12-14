use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut rules_parsed = false;
    let mut middle_sum = 0u32;
    'outer: for line in io::stdin().lines().filter_map(Result::ok) {
        if !rules_parsed {
            if line == "" {
                rules_parsed = true;
                continue;
            }
            let mut parts = line.split('|');
            let first = parts.next().unwrap().parse().unwrap();
            let second = parts.next().unwrap().parse().unwrap();
            rules
                .entry(first)
                .or_insert_with(|| HashSet::new())
                .insert(second);
        } else {
            let parts = line.split(',');
            let mut found_pages: Vec<u8> = Vec::new();
            for part in parts {
                let page = part.parse().unwrap();
                let illegal_pages = rules.entry(page).or_insert_with(|| HashSet::new());
                for found_page in &found_pages {
                    if illegal_pages.contains(found_page) {
                        continue 'outer;
                    }
                }
                found_pages.push(page)
            }
            middle_sum += *found_pages.get(found_pages.len() / 2).unwrap() as u32;
        }
    }
    println!("{middle_sum}");
}
