#![allow(non_snake_case)]
use std::io;

// "S1", "H2"などを数字の部分のみを比較して結果を返す関数。
// mx, nyに対して、x < yを返す
// なお、x,yは1..9であるとする。
fn cmp(s1: &str, s2: &str) -> bool {
    let d1: i32 = s1.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
    let d2: i32 = s2.chars().nth(1).unwrap().to_digit(10).unwrap() as i32;
    d1 < d2
}


/// バブルソート。
fn bubble_sort(a: &mut [&str]) {
    let n = a.len();
    for i in 0..n {
        for j in 1..(n-i) {
            if cmp(&a[j], &a[j-1]) {
                a.swap(j, j-1);
            }
        }
    }
}

// 挿入ソート。非安定
fn selection_sort(a: &mut [&str]) {
    let n = a.len();
    for i in 1..n {
        let mut min_idx = i-1;
        for j in i..n {
            if cmp(&a[j], &a[min_idx]) {
                min_idx = j;
            }
        }
        a.swap(min_idx, i-1);
    }
}

fn print_vec(a: &[&str]){
    let s = a.join(" ");
    println!("{}", s);
}

fn same_vec(a1: &[&str], a2: &[&str]) -> bool {
    let n = a1.len();
    if n != a2.len() {
        return false;
    } else {
        for i in 0..n {
            if a1[i] != a2[i] {
                return false;
            }
        }
    }
    true
}

fn main(){
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n: i32 = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut a1: Vec<&str> = buf.split_whitespace().collect();
    let mut a2 = a1.clone();

    // let mut a = vec!["H4", "C9", "S4", "D2", "C3"];
    // let mut a1: Vec<String> = a.iter().map(|&e| e.to_string()).collect();
    // let mut a2: Vec<String> = a1.clone();

    // print_vec(&a1);
    bubble_sort(&mut a1);
    print_vec(&a1);
    println!("Stable");     // バブルソートは常に安定

    selection_sort(&mut a2);
    print_vec(&a2);
    if same_vec(&a1, &a2) { // バブルソートと比較して、異なったら非安定とする
        println!("Stable");
    } else {
        println!("Not stable");
    }
}
