#![allow(dead_code)]

use std::io::{self, Read};
use std::str::FromStr;
use std::fmt::{Display,Debug};
use std::cmp::PartialOrd;
use std::default::Default;


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

/// ヒープで構成された優先度付きキュー
/// - 左右の子は親より小さい
/// - 一番低いレベルの葉は左から順に隙間なく埋まっている
struct PriorityQueue<T: Display + PartialOrd + Default + Debug + Clone> {
    vals: Vec<T>,   // 添え字は1開始とする
    last: usize,    // 末尾の値の添え字を指す。0の時に空となる
}

impl<T: Display + PartialOrd + Default + Debug + Clone> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue {
            vals: vec![T::default()],
            last: 0,
        }
    }

    fn right(&self, i: usize) -> Option<usize> {
        let id = i*2+1;
        if id <= self.last {
            Some(id)
        } else {
            None
        }
    }

    fn left(&self, i: usize) -> Option<usize> {
        let id = i*2;
        if id <= self.last {
            Some(id)
        } else {
            None
        }
    }

    fn parent(&self, i: usize) -> Option<usize> {
        if i <= 1 {
            None
        } else {
            Some(i/2)
        }
    }

    fn extract_max(&mut self) -> Option<T> {
        if self.last < 1 {
            None
        } else {
            let res = self.vals[1].clone();
            self.vals.swap(1,self.last);
            self.last -= 1;

            // 根から葉にかけて整理
            let mut p = 1;
            while let Some(c) = self.arrange_node(p) {
                p = c;
            }

            Some(res)
        }
    }

    fn insert(&mut self, val: T) {
        self.insert_last(val);
        self.arrange_from_bottom();
    }

    fn insert_last(&mut self, val: T) {
        self.last += 1;
        if self.last < self.vals.len() {
            self.vals[self.last] = val; 
        } else {
            self.vals.push(val);
        }
    }

    fn arrange_from_bottom(&mut self) {
        let mut l = self.last;
        // 末尾の葉から根に向けて値を整理
        while let Some(p) = self.parent(l) {
            self.arrange_node(p);
            l = p;
        }
    }

    /// idとその子の値を整理する
    /// 入れ替えが発生したとき、入替先となった子の添え字を返す
    fn arrange_node(&mut self, id: usize) -> Option<usize> {
        // 親と右の子を比較
        let mut largest = match self.right(id) {
            None => {id},
            Some(r) => { 
                if self.vals[id] < self.vals[r] {
                    r
                } else {
                    id
                }
            },
        };

        // 親・右で大きい値と左の子を比較
        largest = match self.left(id) {
            None => {largest},
            Some(l) => { 
                if self.vals[largest] < self.vals[l] {
                    l
                } else {
                    largest
                }
            },
        };

        if largest != id {
            // 3ノードで一番大きい値が親でない時
            self.vals.swap(id, largest);
            Some(largest)
        } else {
            None
        }
    }

    fn print(&self) {
        if self.vals.len() > 1 {
            println!("{}", self.vals[1..self.last+1].iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" "));
        }
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let mut pq = PriorityQueue::<i64>::new();

    loop {
        let cmd: String = sc.read();

        match cmd.as_str() {
            "insert" => {
                let val: i64 = sc.read();
                pq.insert(val);
            },
            "extract" => {
                if let Some(m) = pq.extract_max() {
                    println!("{}", m);
                }
            },
            "end" => {
                break;
            },
            _ => {
                unreachable!();
            }
        }
    }
}
