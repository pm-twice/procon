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
    find_shortest(&mat, &mut dist);

    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
