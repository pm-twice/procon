#![allow(non_snake_case)]

use std::io;
use std::f64::consts;

fn main() {
    let mut buf = String::new();
    let sin = io::stdin();
    sin.read_line(&mut buf).ok();
    let r: f64 = buf.trim().parse().unwrap();

    println!("{:.6} {:.6}", consts::PI*r*r, 2.0*consts::PI*r);
}
