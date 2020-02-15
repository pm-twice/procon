#![allow(non_snake_case)]

use std::io;

// std::i32::MAX ≒ 2e9なので途中でも用いて問題なさそう

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut stack: Vec<i32> = vec![];

    for v in buf.split_whitespace() {
        // println!("{}", &v);
        match v.parse::<i32>() {
            Ok(ok) => stack.push(ok),
            Err(_) => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    let s = v.to_string();
                    match s.as_str() {
                        "+" => stack.push(a+b),
                        "-" => stack.push(b-a),
                        "*" => stack.push(a*b),
                        _ => panic!("not match"),
                }
            }
        }
    }
    let res = stack.pop().unwrap();
    println!("{}", res);
}
