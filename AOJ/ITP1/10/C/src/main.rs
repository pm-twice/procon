#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();

    loop{
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let n = buf.trim().parse::<u64>().unwrap();
        if n == 0 {
            break;
        }
        let n = n as f64;

        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let scores = buf.split_whitespace()
            .map(|v| v.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        let sum = scores.iter().fold(0.0, |s, x| s + x);
        let mean = sum / n;
        let sigma = scores.iter().fold(0.0, |v, x| v + ((x-mean).powi(2) / n)).sqrt();

        println!("{}", sigma);
    }
}
