use std::io::{self, Read};
use std::str::FromStr;
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

// プリムのアルゴリズムを使った最小全域木の探索。
// 指定されたidxを開始点として探索を行う。
fn prim(graph: &Vec<Vec<i64>>, idx: usize) -> i64 {
    let n = graph.len();

    // idxからiに対して訪問にかかるコスト
    let mut cost: Vec<i64> = (0..n).map(|i| graph[idx][i]).collect();
    let mut check = vec![false; n]; // 訪問確認用
    check[idx] = true;
    let mut sum = 0;

    for _ in 0..n-1 {
        // 最小のコストを探索
        let mut m = i64::MAX;
        let mut mid = 0;
        for j in 0..n {
            if check[j] == false && m > cost[j] {
                m = cost[j];
                mid = j;
            }
        }

        // 最小コストの点を追加し、その点からの移動コストを次のコストへ反映させる
        sum += m;
        check[mid] = true;
        for j in 0..n {
            if cost[j] > graph[mid][j] {
                cost[j] = graph[mid][j];
            }
        }
    }

    sum
}

fn solve(graph: &Vec<Vec<i64>>) -> i64 {
    // n < 100で全探索でいいのか？n!通りになるのか
    let n = graph.len();
    let mut min = i64::MAX;
    for i in 0..n {
        let m = prim(&graph, i);
        if m < min {
            min = m;
        }
    }
    min
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();

    let graph: Vec<Vec<i64>> = (0..n).map(|_| {
        (0..n).map(|_| { 
            let v = sc.read::<i64>();
            if v == -1 {
                i64::MAX    // -1をMAXに置き換えて比較しやすくする
            } else {
                v
            }
        }).collect()
    }).collect();

    println!("{}", solve(&graph));
}
