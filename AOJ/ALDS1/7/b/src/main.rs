//! 一応解けたが、idが0..n-1のn個であることを読み飛ばしていて、
//! idがn以上のどんな値でも取れるとして解いてしまっているので、idと配列の添え字の変換など大分複雑になっている

use std::io::{self, Read};
use std::str::FromStr;
use std::cmp;

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
    id: u32,
    left_id: Option<u32>,
    right_id: Option<u32>,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    depth: u32,
    height: u32,
}

impl Node {
    fn new(id: u32) -> Node {
        Node {
            id: id,
            left_id: None,
            right_id: None,
            parent: None,
            left: None,
            right: None,
            depth: 0,
            height: 0,
        }
    }

    fn rec_calc_depth(nodes: &mut Vec<Node>, cur_id: usize, cur_depth: u32) -> u32 {
        nodes[cur_id].depth = cur_depth;
        let mut mx = cur_depth;
        if let Some(l) = nodes[cur_id].left {
            let d = Node::rec_calc_depth(nodes, l, cur_depth+1);
            if d > mx {
                mx = d;
            }
        }
        if let Some(r) = nodes[cur_id].right {
            let d = Node::rec_calc_depth(nodes, r, cur_depth+1);
            if d > mx {
                mx = d;
            }
        }
        mx
    }
    
    fn rec_calc_height(nodes: &mut Vec<Node>, cur_id: usize) -> u32 {
        if nodes[cur_id].left.is_none() && nodes[cur_id].right.is_none() {
            nodes[cur_id].height = 0;
            return 0;
        } else if let Some(l) = nodes[cur_id].left {
            let lh = Node::rec_calc_height(nodes, l);
            if let Some(r) = nodes[cur_id].right {
                let rh = Node::rec_calc_height(nodes, r);
                let h = cmp::max(lh, rh) + 1;
                nodes[cur_id].height = h;
                return h;
            } else {
                nodes[cur_id].height = lh + 1;
                return lh + 1;
            }
        } else if let Some(r) = nodes[cur_id].right {
            let rh = Node::rec_calc_height(nodes, r) + 1;
            nodes[cur_id].height = rh;
            return rh;
        } else {
            panic!("child error");
        }
    }

    // 深さ・高さを計算する関数
    fn calc_depth_height(nodes: &mut Vec<Node>) {
        for i in 0..nodes.len() {
            if nodes[i].parent.is_none() {
                let _mx_depth = Node::rec_calc_depth(nodes, i, 0);
                let _mx_heigt = Node::rec_calc_height(nodes, i);
            }
        }
    }

    fn get_sib(nodes: &Vec<Node>, cur_id: usize) -> Option<usize> {
        if let Some(p) = nodes[cur_id].parent {
            if let Some(r) = nodes[p].right {
                if r == cur_id {
                    return nodes[p].left;
                } else {
                    return nodes[p].right;
                }
            } else if let Some(l) = nodes[p].left {
                if l == cur_id {
                    return nodes[p].right;
                } else {
                    return nodes[p].left;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn get_degree(&self) -> u32 {
        let mut deg = 0;
        if self.left.is_some(){
            deg += 1;
        }
        if self.right.is_some(){
            deg += 1;
        }
        deg
    }

    fn get_type(&self) -> String {
        if self.parent.is_none() {
            String::from("root")
        } else if self.right.is_some() || self.left.is_some() {
            String::from("internal node")
        } else {
            String::from("leaf")
        }
    }

    fn id2idx(nodes: &Vec<Node>, id: Option<u32>) -> Option<usize> {
        if let Some(id) = id {
            for i in 0..nodes.len() {
                if nodes[i].id == id {
                    return Some(i);
                }
            }
        }
        None
    }

    // parent,left,rightのindexをセットする。len<=25なのでO(n^2)で問題ない
    fn set_idx(nodes: &mut Vec<Node>) {
        // 左右のidを配列におけるindexに変換して登録する
        let n = nodes.len();
        for i in 0..n {
            nodes[i].right = Node::id2idx(nodes, nodes[i].right_id);
            nodes[i].left = Node::id2idx(nodes, nodes[i].left_id);
        }

        // 自身の子に対して、自身の配列中におけるindexをparentとして登録する
        for i in 0..n {
            if let Some(r) = nodes[i].right_id {
                for j in 0..n {
                    if nodes[j].id == r {
                        nodes[j].parent = Some(i);
                        break;
                    }
                }
            }
            if let Some(l) = nodes[i].left_id {
                for j in 0..n {
                    if nodes[j].id == l {
                        nodes[j].parent = Some(i);
                        break;
                    }
                }
            }
        }
    }

    fn print_nodes(nodes: &Vec<Node>) {
        for i in 0..nodes.len() {
            let node = &nodes[i];
            print!("node {}: ", node.id);
            print!("parent = {}, ", match node.parent {
                Some(p) => p.to_string(),
                None => String::from("-1"),
            });
            print!("sibling = {}, ", match Node::get_sib(nodes, i) {
                Some(s) => s.to_string(),
                None => String::from("-1"),
            });
            print!("degree = {}, ", node.get_degree());
            print!("depth = {}, ", node.depth);
            print!("height = {}, ", node.height);
            print!("{}", node.get_type());

            println!("");
        }
    }
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let mut nodes: Vec<Node> = (0..n).map(|i| Node::new(i as u32)).collect();

    for i in 0..n {
        let id: u32 = sc.read();
        let left: i32 = sc.read();
        let right: i32 = sc.read();
        nodes[i].id = id;
        nodes[i].left_id = match left {
            -1 => None,
            _ => Some(left as u32),
        };
        nodes[i].right_id = match right {
            -1 => None,
            _ => Some(right as u32),
        };
    }
    nodes.sort_by_key(|v| v.id);
    Node::set_idx(&mut nodes);
    Node::calc_depth_height(&mut nodes);
    Node::print_nodes(&nodes);
}
