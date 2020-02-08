#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;

/// num回シャッフルした文字列を返す
/// シャッフル: 文字列の前からnum個を取り出し、後ろに連結する
fn shuffle(s: &String, num: u32) -> String {
    // 文字列を2つ連結し、ループを模す。
    let n = s.len();
    let mut res: String = s.clone();
    res.push_str(s);
    let st: usize = num as usize % n;
    res[st..st+n].to_string()
}

fn main() {
    let sin = io::stdin();
    let mut lines = sin.lock().lines();

    loop {
        let word: String = lines.next().unwrap().unwrap();
        if word == "-" {
            break;
        }
        let num: u32 = lines.next().unwrap().unwrap().parse().unwrap();
        let mut sh = 0;
        for _ in 0..num {
            sh += lines.next().unwrap().unwrap().parse().unwrap();
        }
        let res = shuffle(&word, sh);
        println!("{}", res);
    }
}
