#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();
        let x: i32 = ws.next().unwrap().parse().unwrap();
        let y: i32 = ws.next().unwrap().parse().unwrap();

        if x == 0 && y == 0 {
            break;
        } else if x < y{
            println!("{} {}", x, y);
        } else {
            println!("{} {}", y, x);
        }
    }
}
