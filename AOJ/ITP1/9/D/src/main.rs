#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

fn main() {
    let sin = io::stdin();
    let mut lines = sin.lock().lines();

    let mut word: String = lines.next().unwrap().unwrap();
    let _n: u32 = lines.next().unwrap().unwrap().parse().unwrap();

    for line in lines {
        let l = line.unwrap();
        let mut ws = l.split_whitespace();

        let com = ws.next().unwrap();
        let a: usize = ws.next().unwrap().parse().unwrap();
        let b: usize = ws.next().unwrap().parse::<usize>().unwrap() + 1;

        match com {
            "print" => {
                println!("{}", &word[a..b]);
            },
            "reverse" => {
                let trg = word[a..b].chars().rev().collect::<String>();
                // word.replace_range(a..b, trg);  // from 1.27
                for _ in a..b {
                    word.remove(a);
                }
                word.insert_str(a, &trg);
            },
            "replace" => {
                let trg = ws.next().unwrap();
                // word.replace_range(a..b, trg);   // from 1.27
                for _ in a..b {
                    word.remove(a);
                }
                word.insert_str(a, &trg);
            },
            _ => {
                panic!();
            }
        }
    }
}
