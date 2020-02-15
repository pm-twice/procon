use std::io;
use std::fmt;

fn print_vec<U: fmt::Display>(a: &[U]) {
    let s: String = a.iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .connect(" ");
    println!("{}", s);
}

fn insert_sort(a: &mut [i32]){
    // i=0は整列済みであるとする
    for i in 1..a.len() {
        let v = a[i];
        let mut j = (i as i32)-1;
        while j >= 0 && a[j as usize] > v {
            a[(j+1) as usize] = a[j as usize];
            j-=1;
        }
        a[(j+1) as usize]=v;
        print_vec(&a);
    }
}

fn main(){
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let _n: usize = buf.trim().parse().unwrap();
    
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    let mut a: Vec<i32> = buf.split_whitespace().map(|e| e.parse().unwrap()).collect();
    print_vec(&a);

    insert_sort(&mut a);
}