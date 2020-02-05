#![allow(non_snake_case)]

use std::io;


fn main() {
    // [b][f][r]=v
    // b:棟, f:フロア, r:部屋, v:人数
    let mut people: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 10]; 3]; 4];

    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let n: u32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();

        let b: usize = ws.next().unwrap().parse().unwrap();
        let f: usize = ws.next().unwrap().parse().unwrap();
        let r: usize = ws.next().unwrap().parse().unwrap();
        let v: i32 = ws.next().unwrap().parse().unwrap();

        people[b-1][f-1][r-1] += v;
    }

    for b in 0..4 {
        for f in 0..3 {
            for r in 0..10 {
                print!(" {}", people[b][f][r]);
            }
            println!("");
        }
        if b == 3 {
            break;
        }
        for _ in 0..20 {
            print!("#");
        }
        println!("");
    }
}
