use std::io::{self, Read};
use std::str::FromStr;
use std::cmp::{Ord, PartialEq};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::Rc;

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

/// Binary Search Treeのノード
#[derive(PartialEq)]
enum Node<T: Ord + Display + Clone> {
    Nil,
    Cons {
        val: Rc<RefCell<T>>,
        left: Rc<RefCell<Node<T>>>,
        right: Rc<RefCell<Node<T>>>,
    }
}

/// Binary Search Treeの根。インターフェースを兼ねている
struct BST<T: Ord + Display + Clone> {
    root: Rc<RefCell<Node<T>>>
}

impl<T: Ord + Display + Clone> BST<T> {
    fn new() -> BST<T> {
        BST {
            root: Rc::new(RefCell::new(Node::Nil)),
        }
    }

    fn find(&self, value: &T) -> bool {
        Node::find(Rc::clone(&self.root), value)
    }

    fn insert(&mut self, value: T) {
        Node::insert(Rc::clone(&self.root), value);
    }

    fn delete(&mut self, value: &T) {
        Node::delete(Rc::clone(&self.root), value);
    }

    fn preorder(&self) {
        Node::preorder(Rc::clone(&self.root));
        println!("");
    }

    fn inorder(&self){
        Node::inorder(Rc::clone(&self.root));
        println!("");
    }
}

impl<T: Ord + Display + Clone> Node<T> {
    fn new(value: T) -> Node<T> {
        Node::Cons {
            val: Rc::new(RefCell::new(value)),
            left: Rc::new(RefCell::new(Node::Nil)),
            right: Rc::new(RefCell::new(Node::Nil)),
        }
    }

    fn insert(node: Rc<RefCell<Node<T>>>, value: T) {
        match *node.borrow_mut() {
            Node::Cons{ref val, ref left, ref right} => {
                if value < *val.borrow() {
                    Self::insert(Rc::clone(left), value);
                } else {
                    Self::insert(Rc::clone(right), value);
                }
            }, 
            ref mut nil => {
                *nil = Node::new(value);
            }
        }
    }

    fn find(node: Rc<RefCell<Node<T>>>, value: &T) -> bool {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                if *value == *val.borrow() {
                    true
                } else if *value < *val.borrow() {
                    Self::find(Rc::clone(left), value)
                } else {
                    Self::find(Rc::clone(right), value)
                }
            }, 
            Node::Nil => {
                false
            }
        }
    }

    fn min(node: Rc<RefCell<Node<T>>>) -> Option<T> {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                if let Node::Cons{..} = *left.borrow() {
                    return Self::min(Rc::clone(left))
                } else {
                    return Some(val.borrow().clone());
                }
            }, 
            Node::Nil => { return None; },
        }
    }

    fn delete_self(node: Rc<RefCell<Node<T>>>) {
        *node.borrow_mut() = Node::Nil;
    }

    fn derive_node(src: Rc<RefCell<Node<T>>>, dst: Rc<RefCell<Node<T>>>) {
        match *src.borrow() {
            Node::Nil => {
                *dst.borrow_mut() = Node::Nil; 
            }, 
            Node::Cons{ref val, ref left, ref right} => {
                *dst.borrow_mut() = Node::Cons{
                    val: Rc::clone(val),
                    left: Rc::clone(left),
                    right: Rc::clone(right),
                };
            }
        }
    }

    fn delete(node: Rc<RefCell<Node<T>>>, value: &T) {
        let mut delete_self = false;
        let mut derive_node = None;

        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                if *value == *val.borrow() {
                    if *left.borrow() == Node::Nil && *right.borrow() == Node::Nil {
                        // 子が両方とも存在しない
                        delete_self = true;
                    } else if *right.borrow() == Node::Nil {
                        // 左の木を繰り上げる
                        derive_node = Some(Rc::clone(left));
                    } else if *left.borrow() == Node::Nil {
                        // 右の木を繰り上げる
                        derive_node = Some(Rc::clone(right));
                    } else {
                        // 両方の子が存在する
                        if let Some(m) = Self::min(Rc::clone(right)) {
                            // 右の木の最小項目を持ってくる
                            Self::delete(Rc::clone(right), &m);
                            *val.borrow_mut() = m;
                        } else {
                            panic!("unreachable");
                        }
                    }
                } else if *value < *val.borrow() {
                    Self::delete(Rc::clone(left), value);
                } else {
                    Self::delete(Rc::clone(right), value);
                }
            }, 
            Node::Nil => {},
        }

        if delete_self {
            Self::delete_self(Rc::clone(&node));
        }
        if let Some(n) = derive_node {
            Self::derive_node(n, Rc::clone(&node));
        }
    }

    fn preorder(node: Rc<RefCell<Node<T>>>)  {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                print!(" {}", *val.borrow());
                Self::preorder(Rc::clone(left));
                Self::preorder(Rc::clone(right));
            }, 
            Node::Nil => {}
        }
    }

    fn inorder(node: Rc<RefCell<Node<T>>>)  {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                Self::inorder(Rc::clone(left));
                print!(" {}", *val.borrow());
                Self::inorder(Rc::clone(right));
            }, 
            Node::Nil => {}
        }
    }
}


fn main() {
    let sin = io::stdin();
    let sin = sin.lock();
    let mut sc = Scanner::new(sin);

    let n: usize = sc.read();
    let mut root = BST::new();

    for _ in 0..n {
        let cmd: String = sc.read();
        match cmd.as_str() {
            "insert" => {
                let val: i64 = sc.read();
                root.insert(val);
            },
            "find" => {
                let val: i64 = sc.read();
                if root.find(&val) {
                    println!("yes");
                } else {
                    println!("no");
                }
            },
            "delete" => {
                let val: i64 = sc.read();
                root.delete(&val);
            }
            "print" => {
                root.inorder();
                root.preorder();
            },
            _ => { panic!("irregular command"); }
        }
    }
}
