#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();

    let a: i32 = ws.next().unwrap().parse().unwrap();
    let b: i32 = ws.next().unwrap().parse().unwrap();
    let c: i32 = ws.next().unwrap().parse().unwrap();

    let mut cnt = 0;
    for i in a..b+1 {
        if c % i == 0 {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
