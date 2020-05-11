use std::io::{self, Read};
use std::str::FromStr;
use std::cmp;
use std::i64;
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

pub struct SegmentTree {
    l: usize,
    vals: Vec<i64>,
}
impl SegmentTree {
    pub fn new(vals: Vec<i64>) -> SegmentTree {
        let n = vals.len();
        let mut l = 1;
        while l < n {
            l *= 2;
        }
        let mut st = SegmentTree {
            l: l,
            vals: vec![i64::MAX; 2 * l - 1],
        };
        for i in 0..n {
            st.vals[i + l - 1] = vals[i];
        }
        for i in (0..l - 1).rev() {
            st.vals[i] = cmp::min(st.vals[i * 2 + 1], st.vals[i * 2 + 2]);
        }
        st
    }
    pub fn update(&mut self, i: usize, val: i64) {
        let mut id = i + self.l - 1;
        self.vals[id] = val;
        while id > 0 {
            id = (id - 1) / 2;
            let lid = id * 2 + 1;
            let rid = lid + 1;
            self.vals[id] = cmp::min(self.vals[lid], self.vals[rid]);
        }
    }
    pub fn find(&self, i: usize, j: usize) -> i64 {
        self.query(i, j, 0, 0, self.l)
    }
    fn query(&self, i: usize, j: usize, k: usize, l: usize, r: usize) -> i64 {
        if k >= self.vals.len() || j <= l || r <= i {
            i64::MAX
        } else if i <= l && r <= j {
            self.vals[k]
        } else {
            let vl = self.query(i, j, k * 2 + 1, l, (r + l) / 2);
            let vr = self.query(i, j, k * 2 + 2, (r + l) / 2, r);
            cmp::min(vl, vr)
        }
    }
}

/// Segtree版。O(qlogn)
fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let q: usize = sc.read();

    let a: Vec<i64> = vec![2i64.pow(31) - 1; n];
    let mut st = SegmentTree::new(a);

    for _ in 0..q { // O(q)
        let com: u32 = sc.read();
        let i: usize = sc.read();

        match com {
            0 => {  // update: O(logn)
                let v: i64 = sc.read();
                st.update(i, v);
            }, 
            1 => {  // find: O(logn)
                let j: usize = sc.read();
                let min = st.find(i,j+1);   // セグ木は[i,j)実装なので、[i,j]→[i,j+1)に変更
                println!("{}", min);
            },
            _ => { panic!("irregular command"); },
        }
    }
}
