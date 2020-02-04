#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let n: u32 = buf.trim().parse().unwrap();

    // 行がs,h,c,dの順で、列が1~13に対応。0は利用しない
    let mut trp: Vec<Vec<u32>> = vec![vec![0; 14]; 4];

    for _ in 0..n {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();
        let s: char = ws.next().unwrap().chars().nth(0).unwrap();
        let d: usize = ws.next().unwrap().parse().unwrap();

        match s {
            'S' => trp[0][d] += 1,
            'H' => trp[1][d] += 1,
            'C' => trp[2][d] += 1,
            'D' => trp[3][d] += 1,
            _ => panic!("irregular suit"),
        };
    }

    for s in 0..4 {
        let suit = match s {
            0 => 'S',
            1 => 'H',
            2 => 'C',
            3 => 'D',
            _ => panic!("irregular suit"),
        };

        for d in 1..14 {
            if trp[s][d] == 0 {
                println!("{} {}", suit, d);
            }
        }
    }
}
