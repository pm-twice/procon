#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut ws = buf.split_whitespace();
    let a: i32 = ws.next().unwrap().parse().unwrap();
    let b: i32 = ws.next().unwrap().parse().unwrap();

    if a < b {
        println!("a < b");
    } else if a > b {
        println!("a > b");
    } else {
        println!("a == b");
    }
}
