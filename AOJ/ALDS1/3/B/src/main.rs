#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();

    let line = l_iter.next().unwrap().unwrap();
    let mut ws = line.split_whitespace();
    let _n = ws.next().unwrap().parse::<usize>().unwrap();
    let q = ws.next().unwrap().parse::<u32>().unwrap();

    let mut queue = VecDeque::new();
    for line in l_iter {
        let l = line.unwrap();
        let mut ws = l.split_whitespace();
        let name: String = ws.next().unwrap().to_string();
        let time: u32 = ws.next().unwrap().parse::<u32>().unwrap();
        queue.push_back((name, time));
    }

    // println!("{:?}", queue);

    let mut t = 0;
    while queue.len() > 0 {
        if let Some((name, time)) = queue.pop_front() {
            if time <= q {
                println!("{} {}", name, t+time);
                t += time;
            } else {
                queue.push_back((name, time - q));
                t += q;
            }
        }
    }
}
