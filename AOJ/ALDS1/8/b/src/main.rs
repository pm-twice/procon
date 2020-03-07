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

enum Tree {
    Nil,
    Node {
        key: i64,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

#[allow(dead_code)]
impl Tree {
    fn insert(&mut self, val: i64) {
        match *self {
            Tree::Nil => {
                *self = Tree::Node{ 
                    key: val, 
                    left: Box::new(Tree::Nil), 
                    right: Box::new(Tree::Nil),
                }
            },
            Tree::Node{ref key, ref mut left, ref mut right} => {
                if *key < val {
                    right.insert(val);
                } else {
                    left.insert(val);
                }
            },
        }
    }

    fn find(&self, val: i64) -> bool {
        if let Tree::Node{ref key, ref left, ref right} = *self {
            if *key == val {
                return true;
            } else if *key < val {
                return right.find(val);
            } else {
                return left.find(val);
            }
        } else {
            return false;
        }
    }

    fn print_pre(&self) {
        if let Tree::Node{ref key, ref left, ref right} = *self {
            print!(" {}", key);
            left.print_pre();
            right.print_pre();
        }
    }
    
    fn print_in(&self) {
        if let Tree::Node{ref key, ref left, ref right} = *self {
            left.print_in();
            print!(" {}", key);
            right.print_in();
        }
    }

    fn print_post(&self) {
        if let Tree::Node{ref key, ref left, ref right} = *self {
            left.print_post();
            right.print_post();
            print!(" {}", key);
        }
    }
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let mut tree = Tree::Nil;

    for _ in 0..n {
        let cmd: String = sc.read();
        match cmd.as_str() {
            "insert" => {
                let val: i64 = sc.read();
                tree.insert(val);
            },
            "find" => {
                let val: i64 = sc.read();
                if tree.find(val) {
                    println!("yes");
                } else {
                    println!("no");
                }
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
