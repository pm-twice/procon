#![allow(dead_code)]
use std::io::{self, Read};
use std::str::FromStr;
use std::u64;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

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

// BinaryHeapで用いるためのStruct
#[derive(Eq, PartialEq)]
struct Node {
    idx: usize,
    cost: u64,
}

impl Ord for Node {
    fn cmp(&self, other :&Node) -> std::cmp::Ordering {
        // Minヒープにするため、比較を左右逆にする(cost小さい、idx小さい)
        // まず、左右逆のcostの比較を行う
        other.cost.cmp(&self.cost)
            // コストが同じだった場合(thenはEqual時のみ呼び出しで、それ以外はそのまま値を返す)、
            // 左右逆のidxの比較を行う
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

// PartialOrdも順序比較に実装が必要。ここではNanは存在しないのでOrdの実装をSomeでラップするだけ
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 二分ヒープを用いたダイクストラ法
fn solve_binary_heap(mat: &Vec<Vec<(usize, u64)>>) -> Vec<u64> {
    let n = mat.len();
    let mut dist = vec![u64::MAX; n];
    let mut par = vec![None; n];    // 親

    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Node {idx: 0, cost: 0,});

    while let Some(Node{idx, cost: _cost}) = heap.pop() {
        for &(i, d) in mat[idx].iter() {
            let s = dist[idx] + d;
            if s < dist[i] {
                dist[i] = s;
                par[i] = Some(idx);
                heap.push(Node {idx: i, cost: dist[i], });
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

    // let dist = solve(&mat);
    let dist = solve_binary_heap(&mat);

    for i in 0..n {
        println!("{} {}", i, dist[i]);
    }
}
