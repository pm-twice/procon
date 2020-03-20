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

fn lcs(x: &String, y: &String) -> u64 {
    // なお、x.len>y.lenであるものとする
    let xlen = x.len();
    let ylen = y.len();
    assert!(xlen >= ylen);

    // 方針: 解説より
    // xi == yi のとき、そのLCSはx[..i]とy[..j]のLCSにxi(=yi)を加えたもの
    // xi != yi のとき、そのLCSはx[..i+1]とy[..j] or x[..i]とy[..j+1]のLCSの長い方となる

    let xc: Vec<char> = x.chars().collect();
    let yc: Vec<char> = y.chars().collect();

    // dp[i][j] をx[0..i+1], y[0..j+1]の最長共通部分列の長さとする
    // X = {x1, x2, ..., xn}, Y = {y1,y2,...,ym}としたとき、
    // dp[i][j]がX[..i+1], Y[..j+1]のLCSの長さとする (つまり添え字0は0埋め)
    let mut dp: Vec<Vec<u64>> = vec![vec![0; ylen+1]; xlen+1];

    for i in 1..xlen+1 {
        for j in 1..ylen+1 {
            if xc[i-1] == yc[j-1] {
                dp[i][j] = dp[i-1][j-1] + 1;
            } else {
                dp[i][j] = if dp[i-1][j] > dp[i][j-1] {
                    dp[i-1][j]
                } else {
                    dp[i][j-1]
                };
            }
        }
    } 

    dp[xlen][ylen]
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    for _ in 0..n {
        let x: String = sc.read();
        let y: String = sc.read();
        let res = if x.len() > y.len () {
            lcs(&x, &y) 
        } else {
            lcs(&y, &x)
        };
        println!("{}", res);
    }
}
