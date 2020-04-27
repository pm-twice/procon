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

// 通常のキューを使った探索。これでも時間内で動作した
fn solve(mat: &Vec<Vec<(usize, u64)>>) -> Vec<u64> {
    let n = mat.len();
    let mut dist = vec![u64::MAX; n];
    dist[0] = 0;

    let mut que = VecDeque::new();
    que.push_back(0);

    while let Some(idx) = que.pop_front() {
        for &(i, d) in mat[idx].iter() {
            let s = dist[idx] + d;
            if s < dist[i] {
                dist[i] = s;
                que.push_back(i);
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

    let mut mat: Vec<Vec<(usize, u64)>> = vec![vec![]; n];
    for _ in 0..n {
        let u: usize = sc.read();
        let k: usize = sc.read();
        for _ in 0..k {
            let v: usize = sc.read();
            let c: u64 = sc.read();
            mat[u].push((v, c));
        }
    }

    let dist = solve(&mat);

    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
