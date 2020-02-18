
use std::io;

/// 線形探索による調査
fn search_same(target: &Vec<u64>, src: &Vec<u64>) -> u32 {
    let mut cnt = 0;
    for v in src {
        for t in target {
            if v == t {
                cnt += 1;
                break;   // 同じsrcに対して複数カウントはしない
            }
        }
    }
    cnt
}

fn main() {
    let sin = io::stdin();
    
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let s = buf.split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _q: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let t = buf.split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("{}", search_same(&s, &t));
}
