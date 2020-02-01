#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut buf = String::new();
    let sin = io::stdin();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let a = ws.next().unwrap().parse::<i32>().unwrap();
    let b = ws.next().unwrap().parse::<i32>().unwrap();

    let d = a / b;
    let r = a % b;
    let f = a as f64 / b as f64;

    println!("{} {} {:.5}", d, r, f);
}
