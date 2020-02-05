use std::io;

fn main(){
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let secs: i32 = buf.trim().parse().unwrap();

    let h: i32 = secs / 3600;
    let m: i32 = (secs % 3600) / 60;
    let s: i32 = secs % 60;

    println!("{}:{}:{}",h,m,s);
}