#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();
        let m: i32 = ws.next().unwrap().parse().unwrap();
        let f: i32 = ws.next().unwrap().parse().unwrap();
        let r: i32 = ws.next().unwrap().parse().unwrap();

        if m == -1 && f == -1 && r == -1 {
            break;
        } else {
            let grade = if m == -1 || f == -1 {
                "F"
            } else {
                let s = m+f;
                match s {
                    80...100 => "A",
                    65...84  => "B",
                    50...64  => "C",
                    30...49  => { if r >= 50 {"C"} else {"D"} },
                    _  => "F",
                }
            };
            println!("{}", grade);
        }
    }
}
