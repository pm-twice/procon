#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let r: usize = ws.next().unwrap().parse().unwrap();
    let c: usize = ws.next().unwrap().parse().unwrap();

    let mut mat: Vec<Vec<i32>> = vec![vec![0; c+1]; r+1];

    for i in 0..r {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        for (j, v) in buf.split_whitespace().enumerate() {
            let d: i32 = v.parse().unwrap();
            mat[i][j] = d;
            mat[r][j] += d;
            mat[i][c] += d;
            mat[r][c] += d;
        }
    }

    for i in 0..r+1 {
        for j in 0..c+1 {
            print!("{}", mat[i][j]);
            if j!=c {
                print!(" ");
            }
        }
        println!("");
    }
}
