#![allow(non_snake_case)]

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;


fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();
    let line = l_iter.next().unwrap().unwrap();

    let mut s: VecDeque<usize> = VecDeque::new();   // 下りの位置
    let mut ponds: VecDeque<(usize,usize)> = VecDeque::new();   // (現在の水たまりの最初の下りの位置, 面積)
    let mut sum = 0;

    for (id, c) in line.chars().enumerate() {
        match c {
            '\\' => { 
                s.push_back(id); 
            },
            '/' => { 
                if let Some(i1) = s.pop_back() {
                    // 上りと同じ高さの対応する下りを用いて、その幅が面積に一致する
                    let w = id - i1;
                    sum += w;

                    if ponds.len() == 0 {
                        ponds.push_back((i1, w));
                    } else {
                        // i1までに存在する水たまりを統合する
                        let mut s_p = 0;
                        while let Some(&(i2, s2)) = ponds.back() {
                            if i2 > i1 {
                                s_p += s2;
                                ponds.pop_back();
                            } else {
                                break;
                            }
                        }
                        ponds.push_back((i1, s_p + w));
                    }
                }
            },
            _ => { ; },
        }
    }

    println!("{}", sum);
    let l = ponds.len(); 
    match l {
        0 => {println!("0");},
        _ => {println!("{} {}", l, ponds.iter().map(|&(_, s)| s.to_string()).collect::<Vec<String>>().join(" "));},
    }
}
