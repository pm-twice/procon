#![allow(dead_code)]
use std::io::{self, Read, Write};
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

// ここでは2次元のkd木を作る
struct Kdtree {
    root: usize,
    points: Vec<(i64, i64, usize)>, // x, y, idx
    rights: Vec<Option<usize>>,
    lefts: Vec<Option<usize>>,
}

impl Kdtree {
    fn new(points: &Vec<(i64, i64)>) -> Kdtree {
        let n = points.len();

        let mut kd = Kdtree {
            root: 0,
            points: (0..n).map(|i| (points[i].0, points[i].1, i)).collect(),
            rights: vec![None; n],
            lefts: vec![None; n],
        };

        let r = kd.make_2dtree(0, n, 0);
        kd.root = r.unwrap();

        kd
    }

    /// O(n logn)のソートを複数回用いて、深さごとにx,yを対象にソートする
    /// 中央値のindexを返す
    fn make_2dtree(&mut self, left: usize, right: usize, depth: u64) -> Option<usize> {
        if !(left < right) {
            return None;
        }

        if depth % 2 == 0 {
            self.points[left..right].sort_by_key(|v| v.0);
        } else {
            self.points[left..right].sort_by_key(|v| v.1);
        }

        let mid: usize = (left + right) / 2;

        self.lefts[mid] = self.make_2dtree(left, mid, depth+1);
        self.rights[mid] = self.make_2dtree(mid+1, right, depth+1);

        Some(mid)
    }

    fn find(&self, sx: i64, tx: i64, sy: i64, ty: i64) -> Vec<usize> {
        let mut results = vec![];
        self._find(&mut results, self.root, sx, tx, sy, ty, 0);
        results.sort();
        results
    }

    fn _find(&self, results: &mut Vec<usize>, idx: usize, sx: i64, tx: i64, sy: i64, ty: i64, depth: u64) {
        let x = self.points[idx].0;
        let y = self.points[idx].1;

        if sx <= x && x <= tx && sy <= y && y <= ty {
            results.push(self.points[idx].2);
        }

        if depth % 2 == 0 {
            if sx <= x && self.lefts[idx].is_some() {
                let l = self.lefts[idx].unwrap();
                self._find(results, l, sx, tx, sy, ty, depth+1);
            } 
            if x <= tx && self.rights[idx].is_some() {
                let r = self.rights[idx].unwrap();
                self._find(results, r, sx, tx, sy, ty, depth+1);
            }
        } else {
            if sy <= y && self.lefts[idx].is_some() {
                let l = self.lefts[idx].unwrap();
                self._find(results, l, sx, tx, sy, ty, depth+1);
            } 
            if y <= ty && self.rights[idx].is_some() {
                let r = self.rights[idx].unwrap();
                self._find(results, r, sx, tx, sy, ty, depth+1);
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

    let kt = Kdtree::new(&points);
    let out = io::stdout();
    let mut out = out.lock();
    for (sx, tx, sy, ty) in squares {
        let results = kt.find(sx, tx, sy, ty);
        for r in results {
            write!(out, "{}\n", r).unwrap();
        }
        writeln!(out, "").unwrap();
    }
}
