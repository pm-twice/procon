#![allow(dead_code)]
use std::io::{self, Read};
use std::str::FromStr;
use std::u64;
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

// 0からidxまでの最短経路を探す
fn find_shortest(mat: &Vec<Vec<u64>>, dist: &mut Vec<u64>) {
    // BFSで探索していく感じ？
    let n = mat.len();
    let mut que = VecDeque::new();
    que.push_back(0);
    dist[0] = 0;

    while let Some(i) = que.pop_front() {
        for j in 0..n {
            if mat[i][j] < u64::MAX {
                let d = dist[i] + mat[i][j];
                if dist[j] > d {
                    dist[j] = d;
                    que.push_back(j);
                }
            }
        }
    }
}

// ダイクストラのアルゴリズムを使った最短経路探索
fn dijkstra(mat: &Vec<Vec<u64>>, dist: &mut Vec<u64>) {
    let n = mat.len();
    // 初期状態
    dist[0] = 0;
    for i in 1..n {
        dist[i] = u64::MAX;
    }

    let mut par: Vec<usize> = vec![0; n];       // 親の添え字
    let mut check: Vec<bool> = vec![false; n];  // 訪問状態

    for _ in 0..n {
        // 探索済みでない最小のd[u]を調査
        let mut min = u64::MAX;
        let mut mid = 0;
        for i in 0..n {
            if check[i] == false && dist[i] <= min {
                mid = i;
                min = dist[i];
            }
        }

        // d[u]を訪問済みとし、未訪問の集合を更新する
        check[mid] = true;
        for i in 0..n {
            if check[i] == false 
                && mat[mid][i] != u64::MAX 
                && dist[mid] + mat[mid][i] < dist[i] {
                par[i] = mid;
                dist[i] = dist[mid] + mat[mid][i];
            }
        }
    }
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let mut mat: Vec<Vec<u64>> = vec![vec![u64::MAX; n]; n];
    for _ in 0..n {
        let u: usize = sc.read();
        let k: usize = sc.read();
        for _ in 0..k {
            let v: usize = sc.read();
            let c: u64 = sc.read();
            mat[u][v] = c;
        }
    }

    // 0からiまでの最短経路
    let mut dist: Vec<u64> = vec![u64::MAX; n];
    // find_shortest(&mat, &mut dist);
    dijkstra(&mat, &mut dist);

    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
