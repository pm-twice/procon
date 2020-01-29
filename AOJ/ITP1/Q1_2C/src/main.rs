#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();

    let mut a: Vec<i32> = buf.split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    a.sort();
    let s = a.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", s);
}
