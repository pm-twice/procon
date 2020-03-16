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

fn left(i: usize) -> usize {
    i*2
}

fn right(i: usize) -> usize {
    i*2+1
}

fn max_heapify(a: &mut Vec<i64>, i: usize, h: usize) {
    let l = left(i);
    let r = right(i);

    // 左の子、自分、右の子で最大のノードを選ぶ
    let mut largest = if l <= h && a[l] > a[i] {
        l
    } else {
        i
    };

    if r <= h && a[r] > a[largest] {
        largest = r;
    }

    if largest != i { // 自身より子の方が大きい場合
        a.swap(i, largest);
        max_heapify(a, largest, h);
    }
}

fn build_max_heap(a: &mut Vec<i64>, h: usize) {
    // h/2で子を持つノードの最後尾から順に計算する
    for i in (1..h/2+1).rev() {
        max_heapify(a, i, h);
    }
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let h: usize = sc.read();
    let mut vals: Vec<i64> = vec![0; h+1];

    for i in 1..h+1 {
        vals[i] = sc.read();
    }
    build_max_heap(&mut vals, h);

    println!(" {}", 
        vals[1..].iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" "));
}
