use std::io;

fn bin_search(target: &Vec<u64>, val: u64) -> bool {
    let mut st = 0;
    let mut end = target.len()-1;
    while st <= end {
        let idx = (end+st) / 2;
        if target[idx] == val {
            return true;
        } else if target[idx] < val {
            st = idx + 1;
        } else {
            end = idx - 1;
        }
    }
    false
}

/// 二分探索による調査。targetは昇順に整列済みであるとする
fn search_same(target: &Vec<u64>, src: &Vec<u64>) -> u32 {
    let mut cnt = 0;
    for v in src {
        if bin_search(target, *v) {
            cnt += 1;
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
