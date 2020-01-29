#![allow(non_snake_case)]

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let w: i32 = ws.next().unwrap().parse().unwrap();
    let h: i32 = ws.next().unwrap().parse().unwrap();
    let x: i32 = ws.next().unwrap().parse().unwrap();
    let y: i32 = ws.next().unwrap().parse().unwrap();
    let r: i32 = ws.next().unwrap().parse().unwrap();

    let mut is_contain = true;
    if x - r < 0 {
        is_contain = false;
    } else if x + r > w {
        is_contain = false;
    } else if y - r < 0 {
        is_contain = false;
    } else if y + r > h {
        is_contain = false;
    }

    if is_contain {
        println!("Yes");
    } else {
        println!("No");
    }

}
