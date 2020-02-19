#![allow(non_snake_case)]

use std::io;

#[allow(dead_code)]
fn gcd(x: u32, y:u32) -> u32 {
    let _x = if x >= y { x } else { y };
    let _y = if x >= y { y } else { x };

    if _y == 1 {
        return 1;
    }
    else if _y == 0 {
        return _x;
    } else {
        return gcd(_y, _x%_y);
    }
}

fn gcd_rev(x: u32, y:u32) -> u32 {
    let mut _x = if x >= y { x } else { y };
    let mut _y = if x >= y { y } else { x };

    while _y > 0 {
        let r = _x % _y;
        _x = _y;
        _y = r;
    }
    _x
}

fn main() {
    let sin = io::stdin();
    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let mut ws = buf.split_whitespace();
    let x: u32 = ws.next().unwrap().parse().unwrap();
    let y: u32 = ws.next().unwrap().parse().unwrap();

    // println!("{}", gcd(x,y));
    println!("{}", gcd_rev(x,y));
}
