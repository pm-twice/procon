use cargo_snippet::snippet;

/// グループ分けを木構造で管理するデータ構造  
/// 要素x,yが存在するとき、次の2操作が高速で行えることが特徴となる
/// 
/// 1. 同じ集合に属するかどうかの判定
/// 2. 両者の集合を合併する
/// 
/// 詳細な解説は[AtCoderの解説](https://atc001.contest.atcoder.jp/tasks/unionfind_a)が分かりやすい
/// 
/// この実装では経路圧縮のみを採用。これで2操作ともだいたい計算量はO(log n)となる
/// 
/// さらにRankの実装を行うと、計算量O(α(n))に減らせる。  
/// (α(n)はアッカーマン関数A(n,n)の逆関数でlogより小さい関数)  
/// こちらについてはそのうちまた別途実装？
/// 
/// # スニペット登録名
/// `snp-unionfind`
/// 
/// # 利用例
/// ```rust
/// use my_rust_snip::set::UnionFind;
/// 
/// let mut uf = UnionFind::new(8);
/// 
/// uf.unite(1,2);
/// uf.unite(3,2);
/// assert_eq!(uf.same(1,3), true);
/// assert_eq!(uf.same(1,4), false);
/// uf.unite(2,4);
/// assert_eq!(uf.same(4,1), true);
/// uf.unite(4,2);
/// uf.unite(0,0);
/// assert_eq!(uf.same(0,0), true);
/// ```
#[snippet("snp-unionfind")]
pub struct UnionFind {
    parent: Vec<usize>, // 自身の親を記録する。親がない場合は自身のindexを保持する
}

#[snippet("snp-unionfind")]
impl UnionFind {
    /// sizeで指定された個数の、それぞれが独立した集合を作成する。  
    /// このとき、配列のindexが集合の要素に対応する
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            parent: (0..size).map(|i| i).collect(),
        }
    }

    /// 指定された要素の根を探す
    /// 同時に要素が属する集合の親を根に接続する
    fn root(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            idx
        } else {
            // 親の探索と同時に根となる親へ参照を変更する
            let p = self.parent[idx];
            let p = self.root(p);
            self.parent[idx] = p;
            p
        }
    }

    /// 指定された2要素が同じ集合に属しているか調べる
    pub fn same(&mut self, id1: usize, id2: usize) -> bool {
        self.root(id1) == self.root(id2)
    }

    /// 指定された2要素が属する集合を合併させる
    pub fn unite(&mut self, id1: usize, id2: usize) {
        let p1 = self.root(id1);
        let p2 = self.root(id2);
        if p1 != p2 {
            self.parent[p1] = p2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::UnionFind;

    #[test]
    fn test_unionfind(){
        let mut uf = UnionFind::new(8);
        uf.unite(1,2);
        uf.unite(3,2);
        assert_eq!(uf.same(1,3), true);
        assert_eq!(uf.same(1,4), false);
        uf.unite(2,4);
        assert_eq!(uf.same(4,1), true);
        uf.unite(4,2);
        uf.unite(0,0);
        assert_eq!(uf.same(0,0), true);
    }
}