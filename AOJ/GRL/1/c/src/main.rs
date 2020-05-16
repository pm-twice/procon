use std::io::{self, Read};
use std::str::FromStr;
use std::i64;
use std::cmp;

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

/// ワーシャルフロイド法を用いた重み付き有向グラフの全対最短経路問題の解決
/// 実行時間は頂点数Vを用いてO(V^3)となる
struct WarshallFloyd {
    n: usize,
    dist: Vec<Vec<i64>>,
}

impl WarshallFloyd {
    /// 与えられた隣接行列を委譲し、構造体を構築
    /// 隣接行列はd[i,i]=0、非接続点はi64::MAXとしている
    fn new(dist: Vec<Vec<i64>>) -> WarshallFloyd {
        WarshallFloyd {
            n: dist.len(),
            dist: dist,
        }
    }

    /// 最短コストの計算を行う
    fn solve(&mut self) {
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    // MAXを含む加算はエラーになるため除外しておく
                    if self.dist[i][k] != i64::MAX && self.dist[k][j] != i64::MAX {
                        self.dist[i][j] = cmp::min(self.dist[i][j], self.dist[i][k] + self.dist[k][j]);
                    }
                }
            }
        }
    }

    fn print(&self) {
        if self.has_negative_close() {
            println!("NEGATIVE CYCLE");
        } else {
            for i in 0..self.n {
                let str = self.dist[i].iter()
                    .map(|v| { match v {
                        &i64::MAX => "INF".to_string(),
                        x => x.to_string(),
                    }})
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", str);
            }
        }
    }

    /// 負の閉路を持っているか判定する
    fn has_negative_close(&self) -> bool {
        for i in 0..self.n {
            if self.dist[i][i] < 0 {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let v: usize = sc.read();   // 頂点数 < 100
    let e: usize = sc.read();   // 辺数 < 9900

    // 重みは2次元行列でよさげ

    let mut dist: Vec<Vec<i64>> = (0..v)
        .map(|_| vec![i64::MAX; v]).collect();
    for i in 0..v {
        dist[i][i] = 0;
    }

    for _ in 0..e {
        let s: usize = sc.read();
        let t: usize = sc.read();
        let d: i64 = sc.read();
        dist[s][t] = d;
    }

    let mut wf = WarshallFloyd::new(dist);
    wf.solve();
    wf.print();
}
