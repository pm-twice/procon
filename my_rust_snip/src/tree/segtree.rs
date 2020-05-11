use cargo_snippet::snippet;
use std::i64;
use std::cmp;

/// SegmentTree (うし木)
/// 
/// 数列に対して、次の2処理を効率よく行えるデータ構造
/// 
/// 1. 数列の任意の位置の値を変更する
/// 2. 数列の一部の区間に何らかの演算をした結果を求める(最小値、総和など)
/// 
/// 上記の処理を、数列の長さNに対してO(logN)で行うことができる。  
/// ただし、構築にはO(N)を要する。
/// 
/// 保持する値の種類により様々な応用ができる。  
/// さらに発展させた遅延SegmentTreeというものも存在する。
/// 
/// 本実装は暫定的にi64のRMQ(最小値)用に実装されている
/// 
/// # スニペット登録名
/// `snp-segtree`
/// 
/// # 利用例
/// ```rust
/// use my_rust_snip::tree::SegmentTree;
/// use std::i64;
/// 
/// let v = vec![i64::MAX; 3];
/// let mut st = SegmentTree::new(v);
/// // assert_eq!(st.vals, vec![i64::MAX; 7]); // inner vals
/// st.update(0, 1);
/// st.update(1, 2);
/// st.update(2, 3);
/// // assert_eq!(st.vals, vec![1,1,3,1,2,3,i64::MAX]); // inner vals
/// 
/// assert_eq!(st.find(0, 2+1), 1); // [0,2] = [0,3)
/// assert_eq!(st.find(1, 2+1), 2); // [1,2] = [1,3)
/// ```
#[snippet("snp-segtree")]
#[snippet(prefix="use std::i64;")]
#[snippet(prefix="use std::cmp;")]
pub struct SegmentTree {
    l: usize,   // nを超える最小の2の累乗値
    vals: Vec<i64>,   // 0-indexの完全二分木の配列実装
}

#[snippet("snp-segtree")]
impl SegmentTree {
    /// 与えられた配列を葉の初期値に持つSegmentTreeを作成する
    pub fn new(vals: Vec<i64>) -> SegmentTree {
        let n = vals.len();
        let mut l = 1;  // nを超える最小の2の累乗を計算する
        while l < n {
            l *= 2;
        }

        // 0-indexなので、親は(i-1)/2, 子は2n+1,2n+2
        // 最小値用の暫定実装なので初期値はi64::MAXとなっている
        let mut st = SegmentTree {
            l: l,
            vals: vec![i64::MAX; 2*l-1],    // 配列長は2N-1となる
        };

        // 葉に値をセット
        for i in 0..n {
            // 葉はn+1から開始
            st.vals[i+l-1] = vals[i];
        }

        // 子の最小値を親に与えていく
        for i in (0..l-1).rev() {
            st.vals[i] = cmp::min(st.vals[i*2+1], st.vals[i*2+2]);
        }

        st
    }

    /// i番目の要素をvalの値で更新し、親に伝搬させていく関数
    pub fn update(&mut self, i: usize, val: i64) {
        // 葉のノード番号を設定
        let mut id = i + self.l-1;
        self.vals[id] = val;

        // 親に伝搬
        while id > 0 {
            id = (id - 1) / 2;  // 0-indexの親
            let lid = id * 2 + 1;
            let rid = lid + 1;
            self.vals[id] = cmp::min(self.vals[lid], self.vals[rid]);
        }
    }

    /// 区間[l,r)を対象とした最小値を返す
    pub fn find(&self, i: usize, j: usize) -> i64 {
        // 開区間[0, n)を対象とするノード0に対して、開区間[i,j)の最小値を問い合わせる
        self.query(i, j, 0, 0, self.l)
    }

    /// 区間[l,r)を対象とするノードkに対して、開区間[i,j)の最小値を取得する
    fn query(&self, i: usize, j: usize, k: usize, l: usize, r: usize) -> i64 {
        if k >= self.vals.len() || j <= l || r <= i {   // ノードkの区間が交わらない
            i64::MAX
        } else if i <= l && r <= j {    // ノードkの区間が[i,j)に完全に含まれる
            self.vals[k]
        } else {    // ノードkの区間が要求区間[i,j)に部分的に含まれる
            // 対象区間を左右の子に振り分けて返す。左右の対象区間は(r+l)/2で分割される2区間
            let vl = self.query(i, j, k*2+1, l, (r+l)/2);
            let vr = self.query(i, j, k*2+2, (r+l)/2, r);
            cmp::min(vl, vr)
        }
    }
}

#[cfg(test)]
mod test {
    use super::SegmentTree;
    use std::i64;

    #[test]
    fn test_build(){
        let v = vec![i64::MAX; 3];
        let mut st = SegmentTree::new(v);
        assert_eq!(st.vals, vec![i64::MAX; 7]); // 4*2-1
        st.update(0, 1);
        st.update(1, 2);
        st.update(2, 3);
        assert_eq!(st.vals, vec![1,1,3,1,2,3,i64::MAX]);

        assert_eq!(st.find(0, 2+1), 1); // [0,2] = [0,3)
        assert_eq!(st.find(1, 2+1), 2); // [1,2] = [1,3)
    }
}