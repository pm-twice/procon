#![allow(non_snake_case)]

use std::io;

fn multi(a: &Vec<Vec<i32>>, b: &Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let m = b.len();

    let mut c: Vec<i32> = vec![0; n];

    for i in 0..n {
        for j in 0..m {
            c[i] += a[i][j] * b[j];
        }
    }

    c
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let m: usize = ws.next().unwrap().parse().unwrap();

    let mut a: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut b: Vec<i32> = vec![0; m];

    for i in 0..n {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        for (j, v) in buf.split_whitespace().enumerate() {
            a[i][j] = v.parse::<i32>().unwrap();
        }
    }

    for i in 0..m {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        b[i] = buf.trim().parse().unwrap();
    }
    
    // println!("{:?}", &a);
    // println!("{:?}", &b);
    let c = multi(&a, &b);

    for v in &c {
        println!("{}", v);
    }
}
