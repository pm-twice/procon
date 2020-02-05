#![allow(non_snake_case)]

use std::io;

// h > 3, w > 3
fn print_frame(h: u32, w: u32) {
    assert!(h >= 3 && w >= 3, "h & w: error");
    
    for y in 0..h {
        for x in 0..w {
            if x == 0 || x == w-1 || y == 0 || y == h-1 {
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
            print_frame(h, w);
            println!("");
        }
    }
}
