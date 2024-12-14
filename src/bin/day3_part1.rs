use std::io::{self, Read};

fn main() {
    let mut sum: u32 = 0;
    let mut state: u32 = 0;
    let mut a_digits: u32 = 0;
    let mut b_digits: u32 = 0;
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    for byte in io::stdin().bytes().filter_map(Result::ok) {
        if byte == b'm' {
            a_digits = 0;
            b_digits = 0;
            a = 0;
            b = 0;
            state = 1;
        } else if state == 1 && byte == b'u' {
            state = 2;
        } else if state == 2 && byte == b'l' {
            state = 3;
        } else if state == 3 && byte == b'(' {
            state = 4;
        } else if state == 4 && byte.is_ascii_digit() && a_digits < 3 {
            a = a * 10 + u32::from(byte - b'0');
            a_digits += 1;
        } else if state == 4 && byte == b',' && a_digits > 0 {
            state = 5;
        } else if state == 5 && byte.is_ascii_digit() && b_digits < 3 {
            b = b * 10 + u32::from(byte - b'0');
            b_digits += 1;
        } else if state == 5 && byte == b')' && b_digits > 0 {
            sum += a * b;
            state = 0;
        } else {
            state = 0;
        }
    }
    println!("{sum}")
}
