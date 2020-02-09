#![allow(non_snake_case)]

use std::io;
use std::f64;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let a: f64 = ws.next().unwrap().parse().unwrap();
    let b: f64 = ws.next().unwrap().parse().unwrap();
    let c: f64 = ws.next().unwrap().parse().unwrap();

    let t: f64 = c / 180.0 * f64::consts::PI;
    let h: f64 = b * f64::sin(t);
    let s: f64 = a * h / 2.0;
    
    // 余弦定理から
    // c = sqrt(a*a + b*b - 2abcosC)
    let _c = (a*a + b*b - 2.0*a*b*f64::cos(t)).sqrt();
    let l: f64 = a + b + _c;

    println!("{}\n{}\n{}", s, l, h);

}
