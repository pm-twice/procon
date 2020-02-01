#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();

    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();
        let a = ws.next().unwrap().parse::<i32>().unwrap();
        let op: &str = ws.next().unwrap();
        let b = ws.next().unwrap().parse::<i32>().unwrap();

        let res = match op {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "?" => break,
            _ => break,
        };
        println!("{}", res);
    }
}
