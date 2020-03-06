use std::io::{self, Read};
use std::str::FromStr;
use std::cmp::Ord;
use std::fmt::Display;

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

enum Tree<T: Ord + Display> {
    Nil,
    Node { 
        key: T, 
        left: Box<Tree<T>>, 
        right: Box<Tree<T>>,
    },
}

impl<T: Ord + Display> Tree<T> {
    fn insert_rec(&mut self, val: T) {
        match *self {
            Tree::Nil => {
                *self = Tree::Node{
                    key: val, 
                    left: Box::new(Tree::Nil), 
                    right: Box::new(Tree::Nil) };
            },
            Tree::Node{ ref key, ref mut left, ref mut right } => {
                if val < *key {
                    left.insert_rec(val);
                } else {
                    right.insert_rec(val);
                }
            },
        }
    }

    fn print_in(&self) {
        match *self {
            Tree::Nil => {;},
            Tree::Node{ ref key, ref left, ref right } => {
                left.print_in();
                print!(" {}", key);
                right.print_in();
            },
        }
    }
    
    fn print_pre(&self) {
        match *self {
            Tree::Nil => {;},
            Tree::Node{ ref key, ref left, ref right } => {
                print!(" {}", key);
                left.print_pre();
                right.print_pre();
            },
        }
    }
} 

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let mut tree : Tree<i64> = Tree::Nil;

    for _ in 0..n {
        let cmd: String = sc.read();
        match cmd.as_str() {
            "insert" => {
                let d: i64 = sc.read();
                tree.insert_rec(d);
            },
            "print" => {
                tree.print_in();
                println!("");
                tree.print_pre();
                println!("");
            },
            _ => { panic!("irregular command"); }
        }
    }
}
