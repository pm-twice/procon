#![allow(non_snake_case)]

use std::io;
use std::f64;

/// ミンコフスキー距離を計算する関数。
/// n > 100のとき、inf扱いして別計算する
fn minkov(a: &Vec<f64>, b: &Vec<f64>, n: i32) -> f64 {
    assert_eq!(a.len(), b.len());

    let z_iter = a.iter().zip(b.iter());
    if n < 100 {
        let mut dis = 0.0;
        for (x, y) in z_iter {
            dis += (x-y).abs().powi(n);
        }
        dis.powf(1.0/ n as f64)
    } else {
        let mut mx = f64::MIN;
        for (x, y) in z_iter {
            let d = (x-y).abs();
            if mx < d {
                mx = d;
            }
        }
        mx
    }
}

fn main() {
    let sin = io::stdin();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n = buf.trim().parse::<u32>().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let x = buf.split_whitespace()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let y = buf.split_whitespace()
        .map(|v| v.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let r_1 = minkov(&x, &y, 1);
    let r_2 = minkov(&x, &y, 2);
    let r_3 = minkov(&x, &y, 3);
    let r_inf = minkov(&x, &y, 1000);

    println!("{:.6}", r_1);
    println!("{:.6}", r_2);
    println!("{:.6}", r_3);
    println!("{:.6}", r_inf);
}
