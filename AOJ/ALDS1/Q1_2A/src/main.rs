#![allow(non_snake_case)]
use std::io;

fn bubble_sort(a: &mut [i32]) -> i32 {
    let mut flag = true;
    let n: usize = a.len();
    let mut cnt = 0;

    let mut j: usize = 1;
    while flag {
        flag = false;
        for i in (j..n).rev() {
            if a[i] < a[i-1] {
                let t = a[i];
                a[i] = a[i-1];
                a[i-1] = t;
                flag = true;
                cnt+=1;
            }
        }
        j+=1;
    }
    cnt
}

fn print_vec(a: &Vec<i32>){
    let s = a.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}

fn main() {
    let sin = io::stdin();

    let mut buf = String::new();
    sin.read_line(&mut buf).unwrap();
    let _n: i32 = buf.trim().parse().unwrap();
    
    let mut buf = String::new();
    sin.read_line(&mut buf).unwrap();
    let mut a: Vec<i32> = buf.trim().split_whitespace().map(|e| e.parse().unwrap()).collect();

    // print_vec(&a);
    let cnt = bubble_sort(&mut a);
    print_vec(&a);
    println!("{}", cnt);
}
