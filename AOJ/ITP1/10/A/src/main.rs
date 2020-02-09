#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let x1: f64 = ws.next().unwrap().parse().unwrap();
    let y1: f64 = ws.next().unwrap().parse().unwrap();
    let x2: f64 = ws.next().unwrap().parse().unwrap();
    let y2: f64 = ws.next().unwrap().parse().unwrap();

    let dis = ((x1-x2).powi(2) + (y1-y2).powi(2)).sqrt();
    println!("{}", dis);
}
