#![allow(dead_code)]
use std::io::{self,Read};
use std::str::FromStr;

pub struct Scanner<R: Read> {
    reader: R,
}
impl<R: Read> Scanner<R> {
    pub fn new(reader: R) -> Scanner<R> {
        Scanner { reader: reader }
    }
    pub fn read<T: FromStr>(&mut self) -> T {
        let s = self
            .reader
            .by_ref()
            .bytes()
            .map(|c| c.expect("failed to read char") as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        s.parse::<T>().ok().expect("failed to parse token")
    }
}

#[derive(Debug)]
struct Node {
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
}

fn rec_find_root(pre_idx: &[usize], in_idx: &[usize], nodes: &mut Vec<Node>, parent: Option<usize>) -> Option<usize> {
    // rootの探索
    let l = pre_idx.len();
    assert_eq!(l, in_idx.len());
    if l == 0 {
        return None;
    } else if l == 1 {
        let r = pre_idx[0];
        nodes[r].parent = parent;
        return Some(r);
    } else {
        // Preorder: root, left, right
        // Inorder: left, root, right

        // rootはpre[0]で確定
        // in におけるrootより左、右で木を分割して再度探索
        let r = pre_idx[0];
        let r_idx = in_idx.iter().position(|x| *x == r).unwrap();

        let nxt_in_left = &in_idx[0..r_idx];
        let nxt_in_right = &in_idx[r_idx+1..l];
        let nxt_pre_left = &pre_idx[1..r_idx+1];
        let nxt_pre_right = &pre_idx[r_idx+1..l];

        nodes[r].parent = parent;
        nodes[r].left = rec_find_root(nxt_pre_left, nxt_in_left, nodes, Some(r));
        nodes[r].right = rec_find_root(nxt_pre_right, nxt_in_right, nodes, Some(r));
        return Some(r)
    }
}

/// Preorder: root, left, right
fn get_pre(nodes: &Vec<Node>, idx: usize, ids: &mut Vec<usize>){
    ids.push(idx+1);   // 添え字対応のため番号は1減少させている
    if let Some(l) = nodes[idx].left {
        get_pre(nodes, l, ids);
    }
    if let Some(r) = nodes[idx].right {
        get_pre(nodes, r, ids);
    }
}

/// Inorder: left, root, right
fn get_in(nodes: &Vec<Node>, idx: usize, ids: &mut Vec<usize>){
    if let Some(l) = nodes[idx].left {
        get_in(nodes, l, ids);
    }
    ids.push(idx+1);   // 添え字対応のため番号は1減少させている
    if let Some(r) = nodes[idx].right {
        get_in(nodes, r, ids);
    }
}

/// Postorder: left, right, root
fn get_post(nodes: &Vec<Node>, idx: usize, ids: &mut Vec<usize>){
    if let Some(l) = nodes[idx].left {
        get_post(nodes, l, ids);
    }
    if let Some(r) = nodes[idx].right {
        get_post(nodes, r, ids);
    }
    ids.push(idx+1);   // 添え字対応のため番号は1減少させている
}

fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    
    // id=1,..,Nなので-1して添え字と一致させておく

    // Preorder: root, left, right
    let pre_idx: Vec<usize> = (0..n).map(|_| sc.read::<usize>()-1).collect();

    // Inorder: left, root, right
    let in_idx: Vec<usize> = (0..n).map(|_| sc.read::<usize>()-1).collect();

    // Tree
    let mut nodes: Vec<Node> = (0..n).map(|_| Node {left: None, right: None, parent: None}).collect();

    let root = rec_find_root(&pre_idx[..], &in_idx[..], &mut nodes, None).unwrap();

    // let mut v: Vec<usize> = vec![];
    // get_pre(&nodes, root, &mut v);
    // println!("{}", v.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" "));

    // let mut v: Vec<usize> = vec![];
    // get_in(&nodes, root, &mut v);
    // println!("{}", v.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" "));
    
    let mut v: Vec<usize> = vec![];
    get_post(&nodes, root, &mut v);
    println!("{}", v.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" "));
}
