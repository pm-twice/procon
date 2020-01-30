#![allow(non_snake_case)]
use std::io;

fn main() {
    let sin = io::stdin();
    let mut cnt = 1;
    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let x: i32 = buf.trim().parse().unwrap();
        if x == 0 {
            break;
        }
        println!("Case {}: {}", cnt, x);
        cnt += 1;
    }
}
