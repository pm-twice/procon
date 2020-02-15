use std::io;
use std::i32;

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();

    // N
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut r: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        r[i] = buf.trim().parse().unwrap();
    }

    let mut minv = r[0];
    let mut maxdf = i32::MIN;
    for i in 1..n {
        if maxdf < r[i] - minv {
            maxdf = r[i] - minv;
        }
        if r[i] < minv { 
            minv = r[i];
        }
    }

    println!("{}", maxdf);
}
