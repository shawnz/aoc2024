use std::io::{self, Read};

fn main() {
    let mut enabled = true;
    let mut sum = 0u32;
    let mut state = 0u32;
    let mut a_digits = 0u32;
    let mut b_digits = 0u32;
    let mut a = 0u32;
    let mut b = 0u32;
    for byte in io::stdin().bytes().filter_map(Result::ok) {
        // mul(...) parser
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
            if enabled {
                sum += a * b;
            }
            state = 0;
        }
        // do()/don't() parser
        else if byte == b'd' {
            state = 6;
        } else if state == 6 && byte == b'o' {
            state = 7;
        } else if state == 7 && byte == b'(' {
            state = 8;
        } else if state == 8 && byte == b')' {
            enabled = true;
            state = 0;
        } else if state == 7 && byte == b'n' {
            state = 9;
        } else if state == 9 && byte == b'\'' {
            state = 10;
        } else if state == 10 && byte == b't' {
            state = 11;
        } else if state == 11 && byte == b'(' {
            state = 12;
        } else if state == 12 && byte == b')' {
            enabled = false;
            state = 0;
        }
        // parse error
        else {
            state = 0;
        }
    }
    println!("{sum}")
}
