#![allow(non_snake_case)]

use std::io;

fn print_vec(a: &Vec<i32>){
    let s = a.iter().map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n = buf.trim().parse::<u32>().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut a: Vec<i32> = buf.split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    a.reverse();
    print_vec(&a);
}
