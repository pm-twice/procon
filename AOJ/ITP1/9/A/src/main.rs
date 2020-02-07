#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

fn read_multi_line(){
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();

    let word: String = l_iter.next().unwrap().unwrap().to_lowercase();

    let mut cnt = 0;
    for l in l_iter {
        let line = l.unwrap();
        if line == "END_OF_TEXT" {
            break;
        }
        for w in line.split_whitespace() {
            if w.to_lowercase() == word {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}

/// 遅くてTLE
fn _too_slow(){
    let sin = io::stdin();
    let mut word = String::new();
    sin.read_line(&mut word).ok();
    word = word.to_lowercase().replace("\n", "");
    let mut cnt = 0;

    loop {
        let mut line = String::new();
        sin.read_line(&mut line).unwrap();
        if line == "END_OF_TEXT" {
            break;
        } else {
            for w in line.split_whitespace() {
                if w.to_lowercase() == word {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}

fn main() {
    read_multi_line();
}
