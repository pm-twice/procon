#![allow(non_snake_case)]

use std::io;

fn print_vec(a: &[i32]){
    let s = a.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}

fn selection_sort(a: &mut [i32]) -> i32 {
    let mut cnt = 0;
    let n: usize = a.len();

    for i in 0..n {
        let mut min_idx = i;
        for j in (i+1)..n {
            if a[j] < a[min_idx] {
                min_idx = j;
            } 
        }

        if min_idx != i {
            cnt += 1;
        }
        a.swap(min_idx, i);
    }
    cnt
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).unwrap();
    let _n = buf.trim().parse::<i32>().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).unwrap();
    let mut a: Vec<i32> = buf.split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // let mut a = vec![5,6,4,2,1,3];
    // print_vec(&a);

    let cnt = selection_sort(&mut a);
    print_vec(&a);
    println!("{}", cnt);
}
