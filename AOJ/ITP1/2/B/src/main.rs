#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();

    let mut ws = buf.split_whitespace();
    let a: i32 = ws.next().unwrap().parse().unwrap();
    let b: i32 = ws.next().unwrap().parse().unwrap();
    let c: i32 = ws.next().unwrap().parse().unwrap();

    if a < b && b < c {
        println!("Yes");
    } else {
        println!("No");
    }
}
