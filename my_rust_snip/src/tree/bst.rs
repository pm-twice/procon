use cargo_snippet::snippet;
use std::cmp::{Ord, PartialEq};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::Rc;

/// Binary Search Tree(BST:二分探索木)を表す構造体
/// 
/// 各ノードが子を持つとき、自身<右かつ左≦自身となるように構築されている
/// 
/// # スニペット登録名
/// `snp-bst`
/// 
/// # 利用ライブラリ
/// `
/// use std::cmp::{Ord, PartialEq};
/// use std::fmt::Display;
/// use std::cell::RefCell;
/// use std::rc::Rc;
/// `
/// 
/// # 使用例
/// ```rust
/// use my_rust_snip::tree::BST;
/// 
/// let mut root: BST<i64> = BST::new();
/// let ins = vec![30,17,88,53,5,20,18,28,27,60];
/// for v in ins {
///     root.insert(v);
/// }
/// assert_eq!(&root.inorder(), &vec![5,17,18,20,27,28,30,53,60,88]);
/// assert_eq!(&root.preorder(), &vec![30,17,5,20,18,28,27,88,53,60]);
/// assert_eq!(&root.postorder(), &vec![5,17,18,20,27,28,53,60,88,30]);
/// 
/// let ins = vec![2000000000,55,63,-1,8];
/// for v in ins {
///     root.insert(v);
/// }
/// assert_eq!(&root.inorder(), &vec![-1,5,8,17,18,20,27,28,30,53,55,60,63,88,2000000000]);
/// assert_eq!(&root.preorder(), &vec![30,17,5,-1,8,20,18,28,27,88,53,60,55,63,2000000000]);
/// assert_eq!(&root.postorder(), &vec![-1,5,8,17,18,20,27,28,53,55,60,63,88,2000000000,30]);
/// 
/// let del = vec![53, 2000000000, 20, 5, 8];
/// for d in del {
///     root.delete(&d);
/// }
/// 
/// assert_eq!(&root.inorder(), &vec![-1,17,18,27,28,30,55,60,63,88]);
/// assert_eq!(&root.preorder(), &vec![30,17,-1,27,18,28,88,60,55,63]);
/// assert_eq!(&root.postorder(), &vec![-1,17,18,27,28,55,60,63,88,30]);
/// ```
#[snippet("snp-bst")]
#[snippet(prefix="use std::cmp::{Ord, PartialEq};")]
#[snippet(prefix="use std::fmt::Display;")]
#[snippet(prefix="use std::cell::RefCell;")]
#[snippet(prefix="use std::rc::Rc;")]
pub struct BST<T: Ord + Display + Clone> {
    root: Rc<RefCell<Node<T>>>
}

/// Binary Search Treeのノード
#[snippet("snp-bst")]
#[derive(PartialEq)]
enum Node<T: Ord + Display + Clone> {
    Nil,
    Cons {
        val: Rc<RefCell<T>>,
        left: Rc<RefCell<Node<T>>>,
        right: Rc<RefCell<Node<T>>>,
    }
}

#[snippet("snp-bst")]
impl<T: Ord + Display + Clone> BST<T> {
    /// 空のBSTを新たに作る関数
    pub fn new() -> BST<T> {
        BST {
            root: Rc::new(RefCell::new(Node::Nil)),
        }
    }

    /// valueがBST内部に存在するか確認する関数
    pub fn find(&self, value: &T) -> bool {
        Node::find(Rc::clone(&self.root), value)
    }

    /// valueをBSTに追加する関数
    pub fn insert(&mut self, value: T) {
        Node::insert(Rc::clone(&self.root), value);
    }

    /// valueを持つノードを削除し、BSTの制約が崩れないようにノードを置き換える関数
    pub fn delete(&mut self, value: &T) {
        Node::delete(Rc::clone(&self.root), value);
    }

    /// Pre-Orderの値をVecにして返す関数
    pub fn preorder(&self) -> Vec<T> {
        let mut v = vec![];
        Node::preorder(Rc::clone(&self.root), &mut v);
        v
    }

    /// In-Orderの値をVecにして返す関数
    pub fn inorder(&self) -> Vec<T> {
        let mut v = vec![];
        Node::inorder(Rc::clone(&self.root), &mut v);
        v
    }

    /// Post-Orderの値をVecにして返す関数
    pub fn postorder(&self) -> Vec<T> {
        let mut v = vec![];
        Node::postorder(Rc::clone(&self.root), &mut v);
        v
    }
}

#[snippet("snp-bst")]
impl<T: Ord + Display + Clone> Node<T> {
    /// valueを持ち、子を持たないノードを作成する
    fn new(value: T) -> Node<T> {
        Node::Cons {
            val: Rc::new(RefCell::new(value)),
            left: Rc::new(RefCell::new(Node::Nil)),
            right: Rc::new(RefCell::new(Node::Nil)),
        }
    }

    /// valueの適切な位置への挿入を行う
    /// 自身の値とvalueを比較し、self<valueなら右、逆なら左の子に値を渡す
    /// 自身がNilのとき、自身を子を持たなずvalueを持つ新規ノードに置き換える
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

    /// 指定された値が存在するか求める再帰関数
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

    /// 最小値を求める再帰関数
    fn min(node: Rc<RefCell<Node<T>>>) -> Option<T> {
        match *node.borrow() {
            Node::Cons{ref val, ref left, right: _} => {
                if let Node::Cons{..} = *left.borrow() {
                    return Self::min(Rc::clone(left))
                } else {
                    return Some(val.borrow().clone());
                }
            }, 
            Node::Nil => { return None; },
        }
    }

    /// 自身を削除し、Nilに置き換える関数
    fn delete_self(node: Rc<RefCell<Node<T>>>) {
        *node.borrow_mut() = Node::Nil;
    }

    /// srcの要素をdstへコピーする関数
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

    /// 指定されたvalueを削除する関数
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

        // borrow checkerを回避するため、上記のmatchを抜けてから自身の削除、子要素の引き継ぎを行う
        if delete_self {
            Self::delete_self(Rc::clone(&node));
        }
        if let Some(n) = derive_node {
            Self::derive_node(n, Rc::clone(&node));
        }
    }

    /// Pre-OrderのVecを作る関数
    fn preorder(node: Rc<RefCell<Node<T>>>, vec: &mut Vec<T>)  {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                let v = &*val.borrow();
                vec.push(v.clone());
                Self::preorder(Rc::clone(left), vec);
                Self::preorder(Rc::clone(right), vec);
            }, 
            Node::Nil => {}
        }
    }

    /// In-OrderのVecを作る関数
    fn inorder(node: Rc<RefCell<Node<T>>>, vec: &mut Vec<T>)  {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                Self::inorder(Rc::clone(left), vec);
                let v = &*val.borrow();
                vec.push(v.clone());
                Self::inorder(Rc::clone(right), vec);
            }, 
            Node::Nil => {}
        }
    }

    /// Post-OrderのVecを作る関数
    fn postorder(node: Rc<RefCell<Node<T>>>, vec: &mut Vec<T>)  {
        match *node.borrow() {
            Node::Cons{ref val, ref left, ref right} => {
                Self::inorder(Rc::clone(left), vec);
                Self::inorder(Rc::clone(right), vec);
                let v = &*val.borrow();
                vec.push(v.clone());
            }, 
            Node::Nil => {}
        }
    }
}



#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_operate(){
        let mut root: BST<i64> = BST::new();
        let ins = vec![30,17,88,53,5,20,18,28,27,60];
        for v in ins {
            root.insert(v);
        }
        assert_eq!(&root.inorder(), &vec![5,17,18,20,27,28,30,53,60,88]);
        assert_eq!(&root.preorder(), &vec![30,17,5,20,18,28,27,88,53,60]);
        assert_eq!(&root.postorder(), &vec![5,17,18,20,27,28,53,60,88,30]);

        let ins = vec![2000000000,55,63,-1,8];
        for v in ins {
            root.insert(v);
        }
        assert_eq!(&root.inorder(), &vec![-1,5,8,17,18,20,27,28,30,53,55,60,63,88,2000000000]);
        assert_eq!(&root.preorder(), &vec![30,17,5,-1,8,20,18,28,27,88,53,60,55,63,2000000000]);
        assert_eq!(&root.postorder(), &vec![-1,5,8,17,18,20,27,28,53,55,60,63,88,2000000000,30]);

        let del = vec![53, 2000000000, 20, 5, 8];
        for d in del {
            root.delete(&d);
        }

        assert_eq!(&root.inorder(), &vec![-1,17,18,27,28,30,55,60,63,88]);
        assert_eq!(&root.preorder(), &vec![30,17,-1,27,18,28,88,60,55,63]);
        assert_eq!(&root.postorder(), &vec![-1,17,18,27,28,55,60,63,88,30]);
    }
}
