use std::io;
use std::io::prelude::*;
use std::u32;

/// トラックk台のそれぞれの最大積載量がpの時、それが実現可能か判定する
fn check(vec: &Vec<u64>, k: u32, p: usize) -> bool {
    // k台のトラックでそれぞれ、荷物をpを超えるまで合計していく。
    // すべてのトラックについて合計が終わったとき、荷物が余っていればfalse, 荷物を載せきれていればtrueを返す

    let n = vec.len();
    let mut i = 0;
    for _ in 0..k {
        // トラック1台ごとに,iから荷物を載せていき、pを超えないところまで載せる
        let mut cur_w = 0;
        while i < n && (cur_w + vec[i]) as usize <= p {
            cur_w += vec[i];
            i += 1;
        }
    }

    // 添え字がnに達するなら末尾まで到達したので載せきることが出来た
    i >= n
}

/// k台のトラックにvecの荷物(n個)を割り当てる。この時トラックの最大積載量Pが最小になる値を求める
/// なお、割り当てにおいては、12|3456|のように、連続する荷物でないと同時に載せられない。
/// 場合の数だと、n-1 C k 通りの組合せが存在するはず。
fn calc_p(k: u32, vec: &Vec<u64>) -> u32 {
    // 二分法を用いるのがいいらしい

    let mut st: usize = 0;
    let mut end: usize = 100000 * 10000 + 1; // 荷物の最大個数*最大重量でpの取りうる最大値
    let mut p: usize = 0;
    while st < end {
        let mid = (st + end) / 2;   // 判定対象のpの計算
        if check(vec, k, mid) {
            p = mid;
            end = mid;
        } else {
            st = mid + 1;
        }
    }

    p as u32
}

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();

    let line = l_iter.next().unwrap().unwrap();
    let mut ws = line.split_whitespace();
    let n: usize = ws.next().unwrap().parse().unwrap();
    let k: u32 = ws.next().unwrap().parse().unwrap();

    let mut vec: Vec<u64> = vec![0; n];
    for i in 0..n {
        let line = l_iter.next().unwrap().unwrap();
        vec[i] = line.trim().parse().unwrap();
    }

    let res = calc_p(k, &vec);

    println!("{}", res);
}
