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

/// 添え字が1から始まる完全二分木。添え字がindex対応
/// node[id]の親はid//2, 左の子はid*2, 右の子はid*2+1となる
struct BinaryHeap {
    last: usize,    // 現在のHeapの末尾のindexを指す(nodes[last]が存在)
    nodes: Vec<Node>,
}

struct Node {
    key: i64,
}

impl BinaryHeap {
    fn new(size: usize) -> BinaryHeap {
        BinaryHeap {
            last: 0,
            nodes: (0..size+1).map(|_| Node { key: 0} ).collect(),
        }
    }

    fn insert(&mut self, val: i64) {
        self.last += 1;
        self.nodes[self.last].key = val;

        // 整列は本問題では未実装
    }

    // 与えられたidの親のidを返す関数
    fn id_parent(&self, id: usize) -> Option<usize> {
        if id / 2 > 0 {
            Some(id/2)
        } else {
            None
        }
    }

    // 与えられたidの左の子のidを返す関数
    fn id_left(&self, id: usize) -> Option<usize> {
        let cid = id*2;
        if cid <= self.last {
            Some(cid)
        } else {
            None
        }
    }

    // 与えられたidの右の子のidを返す関数
    fn id_right(&self, id: usize) -> Option<usize> {
        let cid = id*2+1;
        if cid <= self.last {
            Some(cid)
        } else {
            None
        }
    }

    fn print(&self) {
        for i in 1..self.last+1 {
            print!("node {}: key = {}, ", i, self.nodes[i].key);
            if let Some(pid) = self.id_parent(i) {
                print!("parent key = {}, ", self.nodes[pid].key);
            }
            if let Some(cid) = self.id_left(i) {
                print!("left key = {}, ", self.nodes[cid].key);
            }
            if let Some(cid) = self.id_right(i) {
                print!("right key = {}, ", self.nodes[cid].key);
            }
            println!("");
        }
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let h: usize = sc.read();
    let mut heap = BinaryHeap::new(h);
    for _ in 0..h {
        let v: i64 = sc.read();
        heap.insert(v);
    }
    heap.print();
}
