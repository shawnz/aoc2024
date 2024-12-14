use std::io::{self, Read};

fn move_in_dir(width: usize, height: usize, x: usize, y: usize, dir: u8) -> Option<(usize, usize)> {
    if dir == 0 {
        if y > 0 {
            return Some((x, y - 1));
        }
    } else if dir == 1 {
        if x < width - 1 && y > 0 {
            return Some((x + 1, y - 1));
        }
    } else if dir == 2 {
        if x < width - 1 {
            return Some((x + 1, y));
        }
    } else if dir == 3 {
        if x < width - 1 && y < height - 1 {
            return Some((x + 1, y + 1));
        }
    } else if dir == 4 {
        if y < height - 1 {
            return Some((x, y + 1));
        }
    } else if dir == 5 {
        if x > 0 && y < height - 1 {
            return Some((x - 1, y + 1));
        }
    } else if dir == 6 {
        if x > 0 {
            return Some((x - 1, y));
        }
    } else if dir == 7 {
        if x > 0 && y > 0 {
            return Some((x - 1, y - 1));
        }
    }
    None
}

fn find_mas(
    chars: &Vec<u8>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    dir: u8,
    state: u8,
) -> bool {
    let i = x + y * width;
    let c = *chars.get(i).unwrap();
    if (c == b'M' && state == 0) || (c == b'A' && state == 1) {
        let new_coords = move_in_dir(width, height, x, y, dir);
        if let Some((new_x, new_y)) = new_coords {
            return find_mas(chars, width, height, new_x, new_y, dir, state + 1);
        }
    } else if c == b'S' && state == 2 {
        return true;
    }
    false
}

fn main() {
    let mut appearances = 0u32;
    let mut width = 0usize;
    let mut chars: Vec<u8> = Vec::new();
    for byte in io::stdin().bytes().filter_map(Result::ok) {
        if byte.is_ascii_uppercase() {
            chars.push(byte);
        } else if width == 0 && byte == b'\n' {
            width = chars.len();
        }
    }
    let height = chars.len() / width;
    for x in 0..width - 2 {
        for y in 0..height - 2 {
            if (find_mas(&chars, width, height, x, y, 3, 0)
                || find_mas(&chars, width, height, x + 2, y + 2, 7, 0))
                && (find_mas(&chars, width, height, x + 2, y, 5, 0)
                    || find_mas(&chars, width, height, x, y + 2, 1, 0))
            {
                appearances += 1;
            }
        }
    }
    println!("{appearances}")
}
