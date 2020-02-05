#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();
        let n: u32 = ws.next().unwrap().parse().unwrap();
        let x: u32 = ws.next().unwrap().parse().unwrap();

        if x == 0 && n == 0 {
            break;
        } else {
            let mut cnt = 0;
            for i in 1..n-1 {
                for j in i+1..n {
                    for k in j+1..n+1 {
                        if x == i+j+k {
                            cnt += 1;
                        }
                    }
                }
            }
            println!("{}", cnt);
        }
    }
}
