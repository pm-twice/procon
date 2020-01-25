use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<i32> = buf.split_whitespace().map(|e| e.parse().unwrap()).collect();
    v.sort();
    v.reverse();
    let slice = &v[0..3];
    for e in slice.iter() {
        print!("{} ", e);
    }
    println!("");
}
