use std::io::{self, Read};
use std::str::FromStr;
use std::usize;

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

fn solve(mat: &Vec<(usize, usize)>) -> u64 {
    // 2行列(m,n)と(n,l)の積では、結果となる(m,l)の各値において、n回の乗算と和が計算される。
    // よって、2行列の積ではmnl回の乗算が行われる。

    // 乗算回数が最小となる条件を考える
    // 行列M1M2M3の積なら、(M1M2)M3とM1(M2M3)の比較でよい。
    // 行列M1M2M3M4の積なら、
    // (M1)(M2M3M4), (M1M2)(M3M4), (M1M2M3)(M4)の3通りでコストを比較
    // コストの計算は(Mi)は0、(MiMi+1)ならminimi+1という感じ？

    let n = mat.len();

    // dp[i][j]を次のように定義してみる
    // Mi..Mjの最小乗算回数
    // よってdp[i][j] = 0 (i>=j)
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n]; n];

    // 走査が斜めになるのでちょっと複雑
    for c in 1..n { // 乗算対象の行列数
        for r in 0..n-c {   // 対象の行。行列数が増えるごとに減少する
            let i = r;
            let j = r + c;

            // Mr..Mcの最小値を計算する
            let mut mn = usize::max_value();

            for e in i..j {
                // (Mr..Me)(Me+1..Mc)の最少スコアを考える
                let v = dp[i][e] + dp[e+1][j] + mat[i].0 * mat[e].1 * mat[j].1;
                if v < mn {
                    mn = v;
                }
            }
            dp[i][j] = mn;
        }
    }

    // for i in 0..n {
    //     println!("{:?}", &dp[i]);
    // }

    dp[0][n-1] as u64
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();

    let mat: Vec<(usize, usize)> = (0..n)
        .map(|_| (sc.read(), sc.read()))
        .collect();

    println!("{}", solve(&mat));
}
