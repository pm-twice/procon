use std::io;
use std::io::prelude::*;

/// AOJの疑似コートで指定されていたもの
fn partition(vec: &mut Vec<(char,u32)>, st: usize, end: usize) -> usize {
    let num = vec[end].1;
    let mut i: i32 = st as i32 - 1;

    // st..i+1にnum未満、
    // i+1..endにnum以上の数が入るように操作する
    for j in st..end {
        if vec[j].1 <= num {
            i += 1;
            vec.swap(i as usize, j);
        }
    }
    vec.swap(end, i as usize +1);
    i as usize + 1
}

fn quicksort(vec: &mut Vec<(char,u32)>, st: usize, end: usize) {
    if st < end {
        let q = partition(vec, st, end);
        quicksort(vec, st, q-1);
        quicksort(vec, q+1, end);
    }
}

/// 安定ソートとの結果を比較して安定かどうかを判別
fn is_stable(src: &Vec<(char,u32)>, vec: &Vec<(char,u32)>) -> bool {
    let mut t = src.clone();
    t.sort_by_key( |d| d.1 );
    for (i, j) in t.iter().zip(vec.iter()) {
        if i.0 != j.0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();
    let l = l_iter.next().unwrap().unwrap();
    let n: usize = l.trim().parse().unwrap();
    let mut vec: Vec<(char, u32)> = vec![('0',0);n];

    for i in 0..n {
        let l = l_iter.next().unwrap().unwrap();
        let mut ws = l.split_whitespace();
        let c: char = ws.next().unwrap().chars().next().unwrap();
        let d: u32 = ws.next().unwrap().parse().unwrap();
        vec[i] = (c, d);
    }
    let src = vec.clone();

    // println!("{:?}", vec);
    let end = vec.len() - 1;
    quicksort(&mut vec, 0, end);
    
    if is_stable(&src, &vec) {
        println!("Stable");
    } else {
        println!("Not stable");
    }
    // println!("{:?}", vec);
    // println!("{:?}", src);
    for &(c, i) in vec.iter(){
        println!("{} {}", c, i);
    }
}
