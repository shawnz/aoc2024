use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() {
    let mut rules: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut rules_parsed = false;
    let mut middle_sum = 0u32;
    for line in io::stdin().lines().filter_map(Result::ok) {
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
            let mut removed_pages: Vec<u8> = Vec::new();
            let mut did_remove_pages = false;
            for part in parts {
                let page = part.parse().unwrap();
                let illegal_pages = rules.entry(page).or_insert_with(|| HashSet::new());
                found_pages.retain(|found_page| {
                    if illegal_pages.contains(found_page) {
                        removed_pages.push(*found_page);
                        did_remove_pages = true;
                        return false;
                    }
                    true
                });
                found_pages.push(page);
                found_pages.append(&mut removed_pages);
            }
            if did_remove_pages {
                middle_sum += *found_pages.get(found_pages.len() / 2).unwrap() as u32;
            }
        }
    }
    println!("{middle_sum}");
}
