use std::io;

fn main(){
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let x: i32 = buf.trim().parse().unwrap();
    let ans = x.pow(3);
    println!("{}", ans);
}