#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

fn main() {
    let sin = io::stdin();
    let mut lines = sin.lock().lines();

    let _n: u32 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut s_a = 0;
    let mut s_b = 0;
    for l in lines {
        let ll = l.unwrap();
        let mut ws = ll.split_whitespace();
        let a = ws.next().unwrap();
        let b = ws.next().unwrap();
        if a < b {
            s_b += 3;
        } else if a > b {
            s_a += 3;
        } else {
            s_a += 1;
            s_b += 1;
        }
    }

    println!("{} {}", s_a, s_b);
}
