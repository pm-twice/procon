use std::io::{self, Read};
use std::str::FromStr;
pub struct Scanner<R: Read> {
    reader: R,
}
impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }
    pub fn read<T: FromStr>(&mut self) -> T {
        let s = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.expect("failed to read char") as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        s.parse::<T>().ok().expect("failed to parse token")
    }
}

/// d: 訪問時刻
/// f: 終了時刻
/// idx: 対象の添え字
/// time: 現在時刻
fn dfs(vec: &Vec<Vec<usize>>, d: &mut Vec<Option<u32>>, f: &mut Vec<Option<u32>>, idx: usize, time: u32) -> u32 {
    if d[idx].is_none() {
        let mut time = time + 1;
        d[idx] = Some(time);    // 現在時刻を1つ進め、訪問として記録
        for i in vec[idx].iter() {
            time = dfs(vec, d, f, *i, time);
        }
        time += 1;  // 現在時刻を1つ進め、終了時刻として記録
        f[idx] = Some(time);
        time
    } else {
        // 訪問済みなら何もしない
        time
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let vec: Vec<Vec<usize>> = (0..n).map(|_| {
            let _i: usize = sc.read();
            let k: usize = sc.read();
            (0..k).map(|_| sc.read::<usize>() - 1)  // index変換に1減らす
                .collect::<Vec<usize>>()
        }).collect();
    let mut stamp_start: Vec<Option<u32>> = vec![None; n];   // Noneの時未訪問とする
    let mut stamp_end: Vec<Option<u32>> = vec![None; n];   // Noneの時未訪問とする

    let mut time = 0;
    for i in 0..n {
        time = dfs(&vec, &mut stamp_start, &mut stamp_end, i, time);
    }

    for i in 0..n {
        println!("{} {} {}", i+1, stamp_start[i].unwrap(), stamp_end[i].unwrap());
    }
}
