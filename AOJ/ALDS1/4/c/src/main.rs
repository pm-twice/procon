use std::io;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();
    
    let l = l_iter.next().unwrap().unwrap();
    let _n: usize = l.trim().parse().unwrap();

    let mut hash: HashMap<String, bool> = HashMap::new();

    for line in l_iter {
        let l1 = line.unwrap();
        let mut ws = l1.split_whitespace();
        let cmd = ws.next().unwrap().to_string();
        let val = ws.next().unwrap().to_string();

        match cmd.as_str() {
            "insert" => {
                hash.entry(val).or_insert(true);
            },
            "find" => {
                if let Some(_) = hash.get(&val) {
                    println!("yes");
                } else {
                    println!("no");
                };
            },
            _ => {;},
        }
    }
}
