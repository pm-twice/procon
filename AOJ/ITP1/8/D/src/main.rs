#![allow(non_snake_case)]

use std::io;

fn main() {
    let sin = io::stdin();
    
    let mut s = String::new();
    sin.read_line(&mut s).ok();
    s = s.replace("\n", "");    // remove new line

    let mut p = String::new();
    sin.read_line(&mut p).ok();
    p = p.replace("\n", "");    // remove new line

    let s2 = format!("{}{}", s, s);

    // println!("s : {}", s);
    // println!("p : {}", p);
    // println!("s2: {}", s2);

    if s2.contains(&p) {
        println!("Yes");
    } else {
        println!("No");
    }
}
