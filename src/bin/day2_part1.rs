use std::io;

fn main() {
    let mut safe: u32 = 0;
    'outer: for report in io::stdin().lines().filter_map(Result::ok) {
        let levels = report
            .split_whitespace()
            .map(|level_str| level_str.parse::<u32>().unwrap());
        let mut prev_level: Option<u32> = None;
        let mut increasing: Option<bool> = None;
        for level in levels {
            if let Some(prev_level) = prev_level {
                let did_increase = level > prev_level;
                if let Some(increasing) = increasing {
                    if increasing != did_increase {
                        continue 'outer;
                    }
                } else {
                    increasing = Some(did_increase);
                }
                let diff = level.abs_diff(prev_level);
                if diff < 1 || diff > 3 {
                    continue 'outer;
                }
            }
            prev_level = Some(level);
        }
        safe += 1
    }
    println!("{safe}");
}
