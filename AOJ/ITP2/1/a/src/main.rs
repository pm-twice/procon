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

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let q: usize = sc.read();
    let mut vals: Vec<i64> = vec![0; q];
    let mut idx: Option<usize> = None; 

    for _ in 0..q {
        let c: u32 = sc.read();
        match c {
            0 => {  // pushBack
                let x: i64 = sc.read();
                if let Some(i) = idx {
                    vals[i+1] = x;
                    idx = Some(i+1);
                } else {
                    vals[0] = x;
                    idx = Some(0);
                }
            },
            1 => {  //randomAccess
                let p: usize = sc.read();
                println!("{}", vals[p]);
            },
            2 => {  // popBack
                if let Some(i) = idx {
                    idx = Some(i-1);
                }
            },
            _ => {
                panic!("irregular query");
            }
        }
    }
}
