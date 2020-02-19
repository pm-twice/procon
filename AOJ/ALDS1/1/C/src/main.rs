#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

/// 素因子判定により高速化
fn is_prime(num: u32) -> bool {
    if num == 2 {
        return true;
    } else if num < 2 || num % 2 == 0 {
        return false;
    }

    let mut i = 3;
    let end = (num as f64).sqrt() as u32;
    while i <= end {
        if num % i == 0 {
            return false;
        }
        i += 2;
    }
    return true;
}

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();

    let _n: usize = l_iter.next().unwrap().unwrap().trim().parse().unwrap();

    let mut cnt = 0;
    for line in l_iter {
        let l = line.unwrap();
        let d: u32 = l.trim().parse().unwrap();
        if is_prime(d) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
