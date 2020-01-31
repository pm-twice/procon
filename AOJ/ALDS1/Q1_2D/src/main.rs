#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::fmt;
use std::io;

fn insertion_sort(a: &mut Vec<i32>, step: usize) -> i32 {
    let n = a.len();
    let mut cnt = 0;
    for i in step..n {
        let v = a[i];
        let mut j = i;
        while j >= step && a[j-step] > v {
            a[j] = a[j-step];
            j -= step;
            cnt += 1;
        }
        a[j] = v;
    }
    cnt
}

fn shell_sort(a: &mut Vec<i32>) -> (i32, usize, Vec<usize>) {
    let mut cnt = 0;
    let mut m = 1;
    let mut g_vec = vec![];
    loop {
        g_vec.push(m);
        m = 3*m+1;
        if m > a.len() {
            break;
        }
    }
    let m = g_vec.len();
    g_vec.reverse();
    for g in &g_vec {
        cnt += insertion_sort(a, *g);
    } 
    (cnt, m, g_vec)
}

fn print_vec<T: std::fmt::Display>(a: &Vec<T>){
    let s = a.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", s);
}


fn main() {
    // let mut a = vec![5, 1, 4, 3, 2];
    // print_vec(&a);  

    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let n = buf.trim().parse::<usize>().unwrap();
    let mut a: Vec<i32> = vec![0; n];
    for i in 0..n {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        a[i] = buf.trim().parse::<i32>().unwrap();
    }

    let (cnt, m, g) = shell_sort(&mut a);
    // let cnt = insertion_sort(&mut a,2);
    println!("{}", m);
    print_vec(&g);
    println!("{}", cnt);
    //print_vec(&a);
    for v in &a {
        println!("{}", v);
    }
}
