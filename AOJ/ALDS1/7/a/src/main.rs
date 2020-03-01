use std::io;
use std::io::prelude::*;
use std::io::{BufWriter,Write};

// 左子右兄弟表現を用いる
struct Node {
    id: u32,
    parent: Option<usize>,
    child: Option<usize>,   // 1つ目の子供
    sib: Option<usize>, // 下の兄弟
    depth: u32, 
}

impl Node {
    fn new(id: u32) -> Node {
        Node {
            id: id,
            parent: None,
            child: None,
            sib: None,
            depth: 0,
        }
    }

    fn _rec_calc_depth(nodes: &mut Vec<Node>, cur_depth: u32, cur_id: usize){
        nodes[cur_id].depth = cur_depth;
        if let Some(ch) = nodes[cur_id].child {
            Node::_rec_calc_depth(nodes, cur_depth+1, ch);
        }
        
        if let Some(sib) = nodes[cur_id].sib {
            Node::_rec_calc_depth(nodes, cur_depth, sib);
        }
    }

    fn calc_depth(nodes: &mut Vec<Node>) {
        for i in 0..nodes.len() {
            if nodes[i].parent.is_none() {
                Node::_rec_calc_depth(nodes, 0, i);
                break;
            }
        }
    }

    fn print(nodes: &Vec<Node>) {
        let out = io::stdout();
        let mut out = BufWriter::new(out.lock());

        for node in nodes {
            write!(out, "node {}: ", node.id).unwrap();
            write!(out, "parent = {}, ", match node.parent {
                Some(x) => x.to_string(),
                None => String::from("-1"),
            }).unwrap();
            write!(out, "depth = {}, ", node.depth).unwrap();
            if node.parent.is_none() {
                write!(out, "root, ").unwrap();
            } else if node.child.is_none() {
                write!(out, "leaf, ").unwrap();
            } else {
                write!(out, "internal node, ").unwrap();
            }

            write!(out, "[").unwrap();
            if let Some(c) = node.child {
                write!(out, "{}", c).unwrap();
                let mut sib = nodes[c].sib;
                while let Some(s) = sib {
                    write!(out, ", {}", s).unwrap();
                    sib = nodes[s].sib;
                }
            }
            write!(out, "]\n").unwrap();
        }
    }
}

fn main() {
    let sin = io::stdin();
    let mut l_iter = sin.lock().lines();
    let n: usize = l_iter.next().unwrap().unwrap().trim().parse().unwrap();

    let mut nodes = (0..n)
        .map(|v| Node::new(v as u32))
        .collect::<Vec<Node>>();

    for line in l_iter {
        let l = line.unwrap();
        let vals = l.split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let idx = vals[0];
        if vals[1] > 0 {    // 子供の数を比較
            let childs = &vals[2..];
            nodes[idx].child = Some(childs[0]);   // 1番目の子供

            // i番目以降の子供をi-1番目の兄弟として登録
            for i in 0..vals[1] {
                nodes[childs[i]].parent = Some(idx);
                if i < vals[1] - 1 {
                    nodes[childs[i]].sib = Some(childs[i+1]);
                }
            }
        }
    }
    Node::calc_depth(&mut nodes);
    Node::print(&nodes);
}
