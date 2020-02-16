#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();
    let _n = l_iter.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut que = VecDeque::new();

    for line in l_iter {
        let l = line.unwrap();
        let mut ws = l.split_whitespace();
        match ws.next().unwrap() {
            "insert" => { 
                let d: u32 = ws.next().unwrap().parse().unwrap();
                que.push_front(d);
            },
            "delete" => { 
                let d: u32 = ws.next().unwrap().parse().unwrap();
                let mut res = None;
                for (i, v) in que.iter().enumerate() {
                    if d == *v {
                        res = Some(i);
                        break;
                    }
                }
                if let Some(idx) = res {
                    que.remove(idx);
                }
            },
            "deleteLast" => { 
                que.pop_back(); 
            },
            "deleteFirst" => { 
                que.pop_front(); 
            },
            _ => {;},
        }
    }

    let s = que.iter().map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}
