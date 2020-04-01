use std::io::{self, Read};
use std::str::FromStr;
use std::collections::VecDeque;

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

// vecに記載された有効グラフの接続から、点0から各張点までの最短距離を計算する関数
fn calc_dist(vec: &Vec<Vec<usize>>) -> Vec<i32> {
    let n = vec.len();
    let mut dist: Vec<i32> = vec![-1; n];

    // BFSで探索
    let mut que = VecDeque::new();
    que.push_back((0,0));   // (idx, dist)。始点の距離を0として追加
    while let Some((idx, d)) = que.pop_front() {
        if dist[idx] == -1 {
            dist[idx] = d;
            for p in vec[idx].iter() {
                if dist[*p] == -1 {
                    que.push_back((*p, d+1));
                }
            }
        }
    }

    dist
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();

    let vec: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            let _u: usize = sc.read();
            let k: usize = sc.read();
            (0..k)
                .map(|_| { sc.read::<usize>() - 1 })    // 配列添え字用に1減少
                .collect::<Vec<usize>>()
        }).collect();

    let dist = calc_dist(&vec);
    for i in 0..n {
        println!("{} {}", i+1, dist[i]);
    }
}
