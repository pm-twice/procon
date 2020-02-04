#![allow(non_snake_case)]

use std::io;

fn print_rect(h: u32, w: u32) {
    for _i in 0..h {
        for _j in 0..w {
            print!("#");
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
        }
        print_rect(h, w);
        println!("");
    }
}
