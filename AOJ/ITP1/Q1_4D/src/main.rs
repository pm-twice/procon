#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n = buf.trim().parse::<i32>();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let a: Vec<i64> = buf.split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    let v_max = a.iter().max().unwrap();
    let v_min = a.iter().min().unwrap();
    let v_sum: i64 = a.iter().sum();

    println!("{} {} {}", v_min, v_max, v_sum);
}
