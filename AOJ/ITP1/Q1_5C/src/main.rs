#![allow(non_snake_case)]

use std::io;

fn print_check(h: u32, w: u32){
    for y in 0..h {
        for x in 0..w {
            if (x + y) % 2 == 0{
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    } 
}

fn main() {
    let sin = io::stdin();
    loop {
        let mut buf = String::new();
        sin.read_line(&mut buf).ok();
        let mut ws = buf.split_whitespace();
        let h: u32 = ws.next().unwrap().parse().unwrap();
        let w: u32 = ws.next().unwrap().parse().unwrap();
        if h == 0 && w == 0 {
            break;
        } else {
            print_check(h, w);
            println!("");
        }
    }
}
