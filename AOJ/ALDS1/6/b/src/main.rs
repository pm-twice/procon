use std::io;

/// a[st..end]について、a[st..q] < a[q] < a[q+1..end]となるように操作してqを返す関数
/// なお、分割の基準にはa[end]を用いるものとする
/// こちらは自分が教科書で見ていたやつ
fn partition2(vec: &mut Vec<u32>, st: usize, end: usize) -> usize {
    let num = vec[end];
    let mut i = st;
    let mut j = end-1;

    while i < j {
        while vec[i] <= num {
            i+=1;
        }
        while vec[j] > num && j > 0 {
            j-=1;
        }
        if i < j{
            vec.swap(i, j);
        }
    }
    vec.swap(end, i);
    i
}

/// AOJの疑似コートで指定されていたもの
fn partition(vec: &mut Vec<u32>, st: usize, end: usize) -> usize {
    let num = vec[end];
    let mut i: i32 = st as i32 - 1;

    // st..i+1にnum未満、
    // i+1..endにnum以上の数が入るように操作する
    for j in st..end {
        if vec[j] <= num {
            i += 1;
            vec.swap(i as usize, j);
        }
    }
    vec.swap(end, i as usize +1);
    i as usize + 1
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut a: Vec<u32> = buf.split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let l = a.len() - 1;
    // let idx = partition2(&mut a, 0, l);
    let idx = partition(&mut a, 0, l);

    for (i,v) in a.iter().enumerate() {
        if i == idx {
            print!("[{}]", v);
        } else {
            print!("{}", v);
        }
        if i != l {
            print!(" ");
        } else {
            println!("");
        }
    }
}
