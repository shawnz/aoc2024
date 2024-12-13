use std::io;

fn validate_levels(levels: &Vec<u32>) -> bool {
    let mut prev_level: Option<u32> = None;
    let mut increasing: Option<bool> = None;
    for &level in levels {
        if let Some(prev_level) = prev_level {
            let did_increase = level > prev_level;
            if let Some(increasing) = increasing {
                if increasing != did_increase {
                    return false;
                }
            } else {
                increasing = Some(did_increase);
            }
            let diff = level.abs_diff(prev_level);
            if diff < 1 || diff > 3 {
                return false;
            }
        }
        prev_level = Some(level);
    }
    return true;
}

fn main() {
    let mut safe: u32 = 0;
    for report in io::stdin().lines().filter_map(Result::ok) {
        let levels: Vec<u32> = report
            .split_whitespace()
            .map(|level_str| level_str.parse().unwrap())
            .collect();
        if validate_levels(&levels) {
            safe += 1;
            continue;
        }
        // try problem dampener
        for i in 0..levels.len() {
            let mut dampened_levels: Vec<u32> = Vec::with_capacity(levels.len() - 1);
            for j in 0..levels.len() {
                if i != j {
                    dampened_levels.push(*levels.get(j).unwrap());
                }
            }
            if validate_levels(&dampened_levels) {
                safe += 1;
                break;
            }
        }
    }
    println!("{safe}");
}
