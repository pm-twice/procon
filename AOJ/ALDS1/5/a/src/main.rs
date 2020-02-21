use std::io;

fn is_combinable(vec: &Vec<u32>, target: u32) -> bool {
    dfs(0, vec, 0, target)
}

fn dfs(idx: usize, vec: &Vec<u32>, sum: u32, target: u32) -> bool {
    if idx < vec.len() {
        dfs(idx+1, vec, sum+vec[idx], target) 
            || dfs(idx+1, vec, sum, target)
    } else {
        sum == target
    }
}

fn main() {
    let sin = io::stdin();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _n: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let a: Vec<u32> = buf.split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let _q: usize = buf.trim().parse().unwrap();

    let mut buf = String::new();
    sin.read_line(&mut buf).ok();
    let m: Vec<u32> = buf.split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>();
    
    for v in m {
        if is_combinable(&a, v) {
            println!("yes");
        } else {
            println!("no");
        }
    }
    
}
