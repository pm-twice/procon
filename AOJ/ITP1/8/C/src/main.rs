#![allow(non_snake_case)]

use std::io;
use std::char;

fn main() {
    // 0:a, 1:b, ... 25:z
    let mut cnt_vec: Vec<u32> = vec![0; 26];

    let sin = io::stdin();

    loop {
        let mut buf = String::new();
        match sin.read_line(&mut buf) {
            Ok(n) => if n == 0 { break } else { () },
            Err(_) => panic!(),
        };

        for c in buf.chars() {
            let idx = match c {
                'a'...'z' => Some(c as u8 - 'a' as u8),
                'A'...'Z' => Some(c as u8 - 'A' as u8),
                _ => None,
            };

            match idx {
                Some(i) => cnt_vec[i as usize] += 1,
                None => (),
            };
        }
    }

    for (i, v) in cnt_vec.iter().enumerate() {
        let c = char::from_u32(i as u32 + 'a' as u32).unwrap();
        println!("{} : {}", c, v);
    }
}
