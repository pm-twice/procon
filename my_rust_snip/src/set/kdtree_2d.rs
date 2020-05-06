use cargo_snippet::snippet;

/// 二分探索木を組み合わせたデータ構造で、領域探索に用いる。
/// 
/// 1次元の場合は単なる二分探索木だが、k次元になると木の深さごとにソートの対象が変わって循環する。  
/// 
/// いくつかの実装方法があるが、典型的には次のような平衡kd木を構築する([Wikipediaより](https://ja.wikipedia.org/wiki/Kd%E6%9C%A8))
/// 
/// > 1. 木構造を下降すると共に、分割平面を選択する軸を巡回するようにする。例えば、根においてx軸に垂直な平面とし、根の子ではy軸に垂直な平面とし、根の孫ではz軸に垂直な平面とする、というように軸を巡回するように選択していく。
/// > 2. 各ステップで、分割平面生成で選択される点は、kd木に入れる全ての点の対応する軸の座標値の中央値となる点とする。なお、前提として全ての点の集合がアルゴリズムの先頭で得られるものとする。
/// 
/// ただ、常に中央値を親に選択する必要があるわけではないので、その場合はランダム値などを用いて平衡条件を取り払ったりする。  
/// (実用的にはランダム複数値から中央値選択とかで問題ないことが多いらしい)
/// 
/// O(nlogn)のソートを利用した中央値によるkd木の場合、計算量は次の通り
/// 
/// - 木の構築: O(n log^2 n)
/// - 点の挿入: O(log n)
/// - 点の削除: O(log n)
/// - 軸に平行な範囲にある点の探索: O(n^(1-1/d) + k) ※kは報告される点の個数、dはkd木の次元
/// 
/// BSP木の特殊ケースらしい
/// 
/// 本実装は、2次元のkd木を実装している。また、点の挿入・削除は未実装。
/// 
/// ## スニペット名
/// `snp-kdtree2d`
/// 
/// ## 利用例
/// ```rust
/// use my_rust_snip::set::Kdtree2d;
/// 
/// let points = vec![
///     (2,1),
///     (2,2),
///     (4,2),
///     (6,2),
///     (3,3),
///     (5,4),
/// ];
/// let regions = vec![
///     (2,4,0,4),
///     (4,10,2,5),
/// ];
/// 
/// let kdtree = Kdtree2d::new(&points);
/// let ans = vec![
///     vec![0,1,2,4],
///     vec![2,3,5],
/// ];
/// 
/// for i in 0..2 {
///     let (sx, tx, sy, ty) = regions[i];
///     let res = kdtree.find(sx, tx, sy, ty);
///     assert_eq!(res, ans[i]);
/// }
/// ```
#[snippet("snp-kdtree2d")]
pub struct Kdtree2d {
    points: Vec<(i64, i64, usize)>,    // 2次元の点群と元の配列内の位置
    nodes: Vec<Node>,           // 元の配列内のindex,左右の子のindexを記録する配列
}

/// kd木の内部で用いる
/// 配列内index(location)や左右の子を記録するための構造体
/// preorder(root, left, right)順で保存されるものとする。
#[snippet("snp-kdtree2d")]
struct Node {
    location: usize,        // 整列後の配列point内のindex
    right: Option<usize>,   // 右の子のindex。こちらはnode内のindexとなる
    left: Option<usize>,    // 左の子のindex。こちらはnode内のindexとなる
}

#[snippet("snp-kdtree2d")]
impl Kdtree2d {
    /// 点群から2次元のkd木を作成する
    pub fn new(points: &Vec<(i64, i64)>) -> Kdtree2d {
        let n = points.len();
        let mut kt = Kdtree2d {
            points: (0..n).map(|i| (points[i].0, points[i].1, i)).collect(),
            nodes: Vec::with_capacity(n),
        };
        kt.make_2dtree(0, n, 0);
        kt
    }

    fn make_2dtree(&mut self, left: usize, right: usize, depth: usize) -> Option<usize> {
        if !(left < right) {
            return None;
        }

        if depth % 2 == 0 {
            self.points[left..right].sort_by_key(|v| v.0);
        } else {
            self.points[left..right].sort_by_key(|v| v.1);
        }

        let mid: usize = (left + right)/2;
        let node = Node {
            location: mid,
            left: None,
            right: None,
        };
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.nodes[idx].left = self.make_2dtree(left, mid, depth+1);
        self.nodes[idx].right = self.make_2dtree(mid+1, right, depth+1);

        Some(idx)
    }

    /// 指定された領域(sx, sy), (tx, ty)に含まれる点の元の配列内indexを返す関数
    pub fn find(&self, sx: i64, tx: i64, sy: i64, ty: i64) -> Vec<usize> {
        let mut results = vec![];
        self.find_internal(0, sx, tx, sy, ty, 0, &mut results);
        results.sort();
        results
    }

    fn find_internal(&self, node_idx: usize, sx: i64, tx: i64, sy: i64, ty: i64, depth: usize, results: &mut Vec<usize>) {
        let node = &self.nodes[node_idx];
        let idx = node.location;
        let x = self.points[idx].0;
        let y = self.points[idx].1;

        if sx <= x && x <= tx && sy <= y && y <= ty {
            results.push(self.points[idx].2);
        }

        if depth % 2 == 0 {
            if let Some(l_idx) = node.left {
                if x >= sx {
                    self.find_internal(l_idx, sx, tx, sy, ty, depth+1, results);
                }
            }
            if let Some(r_idx) = node.right {
                if x <= tx {
                    self.find_internal(r_idx, sx, tx, sy, ty, depth+1, results);
                }
            }
        } else {
            if let Some(l_idx) = node.left {
                if y >= sy {
                    self.find_internal(l_idx, sx, tx, sy, ty, depth+1, results);
                }
            }
            if let Some(r_idx) = node.right {
                if y <= ty {
                    self.find_internal(r_idx, sx, tx, sy, ty, depth+1, results);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Kdtree2d;

    #[test]
    fn test_2dtree(){
        let points = vec![
            (2,1),
            (2,2),
            (4,2),
            (6,2),
            (3,3),
            (5,4),
        ];
        let regions = vec![
            (2,4,0,4),
            (4,10,2,5),
        ];

        let kdtree = Kdtree2d::new(&points);
        let ans = vec![
            vec![0,1,2,4],
            vec![2,3,5],
        ];

        for i in 0..2 {
            let (sx, tx, sy, ty) = regions[i];
            let res = kdtree.find(sx, tx, sy, ty);
            assert_eq!(res, ans[i]);
        }
    }
}