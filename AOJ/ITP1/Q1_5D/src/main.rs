#![allow(non_snake_case)]

use std::io;

/// 1..nまでの整数のうち、3の倍数および、3が含まれる数字を順番に出力する関数
fn call(n: u32){
    for i in 1..n+1 {
        let mut x = i;
        if x%3==0 {
            print!(" {}", i);
        } else {
            loop {
                if x == 0 {
                    break;
                } else if x%10==3 {
                    print!(" {}", i);
                    break;
                } else {
                    x /= 10;
                }
            }
        }
    }
    println!("");
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let n = buf.trim().parse::<u32>().unwrap();

    call(n);
}
