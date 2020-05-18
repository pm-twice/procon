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

use std::collections::VecDeque;

// 幅優先探索によるトポロジカルソート
fn toploogical_sort(v_num: usize, edges: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut result = Vec::with_capacity(v_num);
    let mut in_num = vec![0; v_num];    // 頂点に対する入力辺の数を表す

    // 入力辺の数を数える
    for &(_,t) in edges.iter() {
        in_num[t] += 1;
    }

    // 入力数0の頂点をQueueに
    let mut que = VecDeque::new();
    for i in 0..v_num {
        if in_num[i] == 0 {
            que.push_back(i);
        }
    }

    // 幅優先探索で入力数0のものから順にresultに追加
    while let Some(v) = que.pop_front() {
        result.push(v);
        // 頂点vから繋がる頂点の入力数を1減らす
        for &(s,t) in edges.iter() {
            if s == v {
                in_num[t] -= 1;
                // 減らした際に入力数が0となったら、Queueに追加する
                if in_num[t] == 0 {
                    que.push_back(t);
                }
            }
        }
    }
    result
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let v: usize = sc.read();
    let e: usize = sc.read();
    let edges: Vec<(usize, usize)> = (0..e)
        .map(|_| {
            (sc.read::<usize>(), sc.read::<usize>())
        }).collect();

    let results = toploogical_sort(v, &edges);
    
    for v in results {
        println!("{}", v);
    }
}
