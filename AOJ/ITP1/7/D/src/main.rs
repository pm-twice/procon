#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();

    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let m: usize = ws.next().unwrap().parse().unwrap();
    let l: usize = ws.next().unwrap().parse().unwrap();

    let mut a: Vec<Vec<u64>> = vec![vec![0; m]; n];
    let mut b: Vec<Vec<u64>> = vec![vec![0; l]; m];
    let mut c: Vec<Vec<u64>> = vec![vec![0; l]; n];

    for i in 0..n {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        for (j, v) in buf.split_whitespace().enumerate() {
            a[i][j] = v.parse::<u64>().unwrap();
        }
    }
    
    for i in 0..m {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        for (j, v) in buf.split_whitespace().enumerate() {
            b[i][j] = v.parse::<u64>().unwrap();
        }
    }

    for i in 0..n {
        for j in 0..l {
            for k in 0..m {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    
    for i in 0..n {
        let s = c[i].iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", s);
    }
}