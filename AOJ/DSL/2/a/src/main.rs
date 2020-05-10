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

/// 出発点にBrute版を実装
fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let q: usize = sc.read();

    let mut a: Vec<u64> = vec![2u64.pow(31) - 1; n];

    for _ in 0..q { // O(q)
        let com: u32 = sc.read();
        let x: usize = sc.read();

        match com {
            0 => {  // update: O(1)
                let y: u64 = sc.read();
                a[x] = y;
            }, 
            1 => {  // find: O(n)
                let y: usize = sc.read();
                let min = a[x..y+1].iter().min();
                println!("{}", min.unwrap());
            },
            _ => { panic!("irregular command"); },
        }
    }
}
