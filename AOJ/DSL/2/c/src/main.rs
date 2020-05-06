#![allow(dead_code)]
use std::io::{self, Read, Write, BufWriter};
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

pub struct Kdtree2d {
    points: Vec<(i64, i64, usize)>,
    nodes: Vec<Node>,
}
struct Node {
    location: usize,
    right: Option<usize>,
    left: Option<usize>,
}
impl Kdtree2d {
    pub fn new(points: &Vec<(i64, i64)>) -> Kdtree2d {
        let n = points.len();
        let mut kt = Kdtree2d {
            points: (0..n).map(|i| (points[i].0, points[i].1, i)).collect(),
            nodes: Vec::with_capacity(n),
        };
        kt.make_2dtree(0, n, 0);
        kt
    }
    fn make_2dtree(&mut self, left: usize, right: usize, depth: usize) -> Option<usize> {
        if !(left < right) {
            return None;
        }
        if depth % 2 == 0 {
            self.points[left..right].sort_by_key(|v| v.0);
        } else {
            self.points[left..right].sort_by_key(|v| v.1);
        }
        let mid: usize = (left + right) / 2;
        let node = Node {
            location: mid,
            left: None,
            right: None,
        };
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.nodes[idx].left = self.make_2dtree(left, mid, depth + 1);
        self.nodes[idx].right = self.make_2dtree(mid + 1, right, depth + 1);
        Some(idx)
    }
    pub fn find(&self, sx: i64, tx: i64, sy: i64, ty: i64) -> Vec<usize> {
        let mut results = vec![];
        self.find_internal(0, sx, tx, sy, ty, 0, &mut results);
        results.sort();
        results
    }
    fn find_internal(
        &self,
        node_idx: usize,
        sx: i64,
        tx: i64,
        sy: i64,
        ty: i64,
        depth: usize,
        results: &mut Vec<usize>,
    ) {
        let node = &self.nodes[node_idx];
        let idx = node.location;
        let x = self.points[idx].0;
        let y = self.points[idx].1;
        if sx <= x && x <= tx && sy <= y && y <= ty {
            results.push(self.points[idx].2);
        }
        if depth % 2 == 0 {
            if let Some(l_idx) = node.left {
                if x >= sx {
                    self.find_internal(l_idx, sx, tx, sy, ty, depth + 1, results);
                }
            }
            if let Some(r_idx) = node.right {
                if x <= tx {
                    self.find_internal(r_idx, sx, tx, sy, ty, depth + 1, results);
                }
            }
        } else {
            if let Some(l_idx) = node.left {
                if y >= sy {
                    self.find_internal(l_idx, sx, tx, sy, ty, depth + 1, results);
                }
            }
            if let Some(r_idx) = node.right {
                if y <= ty {
                    self.find_internal(r_idx, sx, tx, sy, ty, depth + 1, results);
                }
            }
        }
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    // 点集合
    let n: usize = sc.read();
    let points: Vec<(i64, i64)> = (0..n).map(|_| {
            let x: i64 = sc.read();
            let y: i64 = sc.read();
            (x, y)
        }).collect();

    // 領域集合
    let q: usize = sc.read();
    let squares: Vec<(i64, i64, i64, i64)> = (0..q).map(|_| {
        let sx: i64 = sc.read();
        let tx: i64 = sc.read();
        let sy: i64 = sc.read();
        let ty: i64 = sc.read();
        (sx, tx, sy, ty)
    }).collect();

    let kt = Kdtree2d::new(&points);
    let out = io::stdout();
    let mut out = BufWriter::new(out.lock());
    for (sx, tx, sy, ty) in squares {
        let results = kt.find(sx, tx, sy, ty);
        for r in results {
            write!(out, "{}\n", r).unwrap();
        }
        writeln!(out, "").unwrap();
    }
}
