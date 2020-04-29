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
    vals: Vec<usize>, // 自身の親を記録する。親がない場合は自身のindexを保持する
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            vals: (0..size).map(|i| i).collect(),
        }
    }

    fn parent(&mut self, idx: usize) -> usize {
        if self.vals[idx] == idx {
            idx
        } else {
            // 親の探索と同時に根となる親へ参照を変更する
            let p = self.vals[idx];
            let p = self.parent(p);
            self.vals[idx] = p;
            p
        }
    }

    fn same(&mut self, id1: usize, id2: usize) -> bool {
        self.parent(id1) == self.parent(id2)
    }

    fn unite(&mut self, id1: usize, id2: usize) {
        let p1 = self.parent(id1);
        let p2 = self.parent(id2);
        if p1 != p2 {
            self.vals[p1] = p2;
        }
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        let com: usize = sc.read();
        let x: usize = sc.read();
        let y: usize = sc.read();

        match com {
            0 => { uf.unite(x, y); },
            1 => { 
                if uf.same(x,y) {
                    println!("1");
                } else {
                    println!("0");
                };
            },
            _ => { panic!("irregular command"); },
        }
    }
}
