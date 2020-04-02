#![allow(dead_code)]
/// Union-Find版・DFS版を両方記載
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

struct UnionFind {
    /// nodes[i] = iなら、そのノードは根
    nodes: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            nodes: (0..size).map(|i| i).collect(),
        }
    }

    fn root(&mut self, idx: usize) -> usize {
        if self.nodes[idx] == idx {
            idx
        } else {
            // 自身の親を根にして経路圧縮
            let p = self.nodes[idx];
            let r = self.root(p);
            self.nodes[idx] = r;
            self.nodes[idx]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) {
        // ランクなし併合
        let x = self.root(x);
        let y = self.root(y);
        if x != y {
            self.nodes[x] = y;
        }
    }
}

/// Union Findで解く場合
fn solve_unionfind() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut nodes = UnionFind::new(n);
    for _ in 0..m {
        let s: usize = sc.read();
        let t: usize = sc.read();
        nodes.unite(s, t);
    }

    let q: usize = sc.read();
    for _ in 0..q {
        let s: usize = sc.read();
        let t: usize = sc.read();
        if nodes.same(s, t) {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

/// グラフを使ってDFSで解く場合
use std::collections::VecDeque;
fn solve_graph(){
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let m: usize = sc.read();

    let mut nodes: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let s: usize = sc.read();
        let t: usize = sc.read();

        nodes[s].push(t);
        nodes[t].push(s);
    }
    let mut color: Vec<u64> = vec![0; n];   // 0が未訪問
    let mut c = 0;

    // dfs
    for i in 0..n {
        if color[i] != 0 {
            continue;
        } 
        c += 1;
        color[i] = c;

        let mut que = VecDeque::new();
        que.push_back(i);
        while let Some(idx) = que.pop_back() {
            for i in nodes[idx].iter() {
                if color[*i] == 0 {
                    color[*i] = color[idx];
                    que.push_back(*i);
                }
            }
        }
    }


    let q: usize = sc.read();
    for _ in 0..q {
        let s: usize = sc.read();
        let t: usize = sc.read();
        if color[s] == color[t] {
            println!("yes");
        } else {
            println!("no");
        }
    }
}

fn main() {
    solve_graph();
    // solve_unionfind();
}
