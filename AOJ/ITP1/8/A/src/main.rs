#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();

    let s = buf.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("");
    print!("{}", s);
}
