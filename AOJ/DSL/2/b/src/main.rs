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

struct SegmentTree {
    n: usize,
    vals: Vec<i64>, // 0-index
}

impl SegmentTree {
    fn new(size: usize) -> SegmentTree {
        // nをsizeを超える最小の2の累乗値とする
        let mut n = 1;
        while n < size {
            n *= 2;
        }

        SegmentTree {
            n: n,
            vals: vec![0; 2*n-1],   // Sumなので0初期化
        }
    }

    // idx番目の値にvalを加える
    fn add(&mut self, idx: usize, val: i64) {
        let mut idx = idx + self.n - 1;
        self.vals[idx] += val;
        while idx > 0 {
            idx = (idx-1)/2;    // parent's index
            let lid = idx*2 + 1;
            let rid = lid + 1;
            self.vals[idx] = self.vals[lid] + self.vals[rid];
        }
    }

    // 閉区間[a,b]の総和を求める
    fn get_sum(&self, a: usize, b: usize) -> i64 {
        self.query(a, b+1, 0, 0, self.n)    // 閉区間→半開区間に変更
    }

    // 半開区間[l,r)を保持するノードkに対して半開区間[a,b)の総和を求める
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if self.vals.len() <= k || r <= a || b <= l {   // ノードkが完全に区間[a,b)の外
            0
        } else if a <= l && r <= b {   // ノードkが完全に区間[a,b)に含まれる
            self.vals[k]
        } else {    // 子に区間を分割して問合せ
            let vl = self.query(a, b, k*2+1, l, (r+l)/2);
            let vr = self.query(a, b, k*2+2, (r+l)/2, r);
            vl + vr
        }
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut st = SegmentTree::new(n);

    for _ in 0..q {
        let com: u32 = sc.read();
        let i: usize = sc.read();

        match com {
            0 => {  // add
                let v: i64 = sc.read();
                st.add(i-1, v);
            },
            1 => {  // sum
                let j: usize = sc.read();
                let sum = st.get_sum(i-1,j-1);
                println!("{}", sum);
            },
            _ => {
                panic!("irregular command");
            }
        }
    }

}
