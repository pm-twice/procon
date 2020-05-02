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

// bruteの場合、計算量はlog(nq)となり(10^10)TLE
fn slove_brute(points: &Vec<(i64, i64)>, squares: &Vec<(i64, i64, i64, i64)>) {
    let n = points.len();
    let out = io::stdout();
    let mut out = out.lock();

    for &(sx, tx, sy, ty) in squares {
        for i in 0..n {
            if sx <= points[i].0 && points[i].0 <= tx
                && sy <= points[i].1 && points[i].1 <= ty {
                write!(out, "{}\n", i);
            }
        }

        writeln!(out, "");   // 最後に空行
    }
}


// KDTreeを用いた解放
fn slove_kdtree(points: &Vec<(i64, i64)>, squares: &Vec<(i64, i64, i64, i64)>) {
    let n = points.len();
    for &(sx, tx, sy, ty) in squares {
        for i in 0..n {
            if sx <= points[i].0 && points[i].0 <= tx
                && sy <= points[i].1 && points[i].1 <= ty {
                println!("{}", i);
            }
        }

        println!("");   // 最後に空行
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

    slove_brute(&points, &squares);
}
