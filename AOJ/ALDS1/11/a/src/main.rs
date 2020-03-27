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

    let n: usize = sc.read();

    // 隣接リストの作成
    let mut adj_list: Vec<Vec<usize>> = vec![];

    for _ in 0..n {
        let _i: usize = sc.read();
        let k: usize = sc.read();
        
        let ls: Vec<usize> = if k == 0 { 
            vec![]
        } else {
            // 0開始のindexに修正
            (0..k).map(|_| sc.read::<usize>()-1).collect()
        };
        adj_list.push(ls); 
    }

    // println!("{:?}", adj_list);

    // 隣接行列の作成
    let mut adj_mat: Vec<Vec<usize>> = vec![vec![0; n]; n];

    for i in 0..n {
        for j in adj_list[i].iter(){
            adj_mat[i][*j] = 1;
        }
    }

    // println!("{:?}", adj_mat);
    for v in adj_mat.iter() {
        let s = v.iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", s);
    }
}
