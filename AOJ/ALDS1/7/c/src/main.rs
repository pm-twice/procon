use std::io::{self, Read};
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

struct Node {
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

fn preorder(nodes: &Vec<Node>, idx: Option<usize>, top: bool){
    if let Some(i) = idx {
        if top {
            println!("Preorder");
        }
        print!(" {}", i);
        preorder(nodes, nodes[i].left, false);
        preorder(nodes, nodes[i].right, false);
    }
}

fn inorder(nodes: &Vec<Node>, idx: Option<usize>, top: bool){
    if let Some(i) = idx {
        if top {
            println!("Inorder");
        }
        inorder(nodes, nodes[i].left, false);
        print!(" {}", i);
        inorder(nodes, nodes[i].right, false);
    }
}

fn postorder(nodes: &Vec<Node>, idx: Option<usize>, top: bool){
    if let Some(i) = idx {
        if top {
            println!("Postorder");
        }
        postorder(nodes, nodes[i].left, false);
        postorder(nodes, nodes[i].right, false);
        print!(" {}", i);
    }
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let mut nodes: Vec<Node> = (0..n)
        .map(|_| Node{ 
            parent: None,
            left: None,
            right: None,
        }).collect();

    for _ in 0..n {
        let id: usize = sc.read();
        let left: i32  = sc.read();
        let right: i32 = sc.read();
        if left != -1 {
            let lf = left as usize;
            nodes[id].left = Some(lf);
            nodes[lf].parent = Some(id);
        }
        if right != -1 {
            let rg = right as usize;
            nodes[id].right = Some(rg);
            nodes[rg].parent = Some(id);
        }
    }

    for i in 0..n {
        if nodes[i].parent.is_none() {
            preorder(&nodes, Some(i), true);
            println!("");
            inorder(&nodes, Some(i), true);
            println!("");
            postorder(&nodes, Some(i), true);
            println!("");
            break;
        }
    }
}
