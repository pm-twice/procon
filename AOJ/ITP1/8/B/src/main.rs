#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();

        let mut s = 0;
        for c in buf.chars() {
            if c.is_digit(10) {
                s += c.to_digit(10).unwrap();
            } else {
                break;
            }
        }
        if s == 0 {
            break;
        }
        println!("{}", s);
    }
}
