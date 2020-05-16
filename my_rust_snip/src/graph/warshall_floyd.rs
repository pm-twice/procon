use cargo_snippet::snippet;
use std::{i64,cmp};

/// ワーシャルフロイド法を用いた重み付き有向グラフの全対最短経路問題を解決するコード
/// 
/// ## 概要
/// 重み付き有向グラフの全点対間最短経路(APSP)の距離の探索に利用される。  
/// ダイクストラ法と異なり、コストが負でも利用できる。  
/// 計算量は$O(|V|^3)$となる。
/// 
/// ### 前提
/// - 頂点i,jの最短コストを2次元配列$A[i,j]$に対応させる
/// - $A^k[i,j]$を頂点$V^k = \{1,2,3,\cdots,k\}$のみを経由する頂点i,jの最短コストとする
///     + この時の経路の一つを$P^k[i,j]$とする
/// 
/// ### アルゴリズム
/// 1. $A^0[i,j]$を次のように設定
///     + $A^0[i,i]=0$
///     + i,jがコストdで直接つながっていれば、$A^0[i,j]=d$
///     + 繋がっていなければ$A^0[i,j]=\infty$
/// 2. $A^k$について、$A^{k-1}$を基に計算する
///     + 「3つの頂点i,j,kを選び、i->k->jの経路がi->jより短ければ更新する」
///     + つまり、$A^k[i,j]=\min(A^{k-1}[i,j], A^{k-1}[i,k]+A^{k-1}[k,j])$
/// 3. $k=V$まで繰り返す
/// 
/// ここで、行列Aはkを保持するため3次元にする必要があるように一見思える。  
/// しかし、$A[k,k]=0$より、
/// $$A^k[i,k]=\min(A^{k-1}[i,k], A^{k-1}[i,k]+A^{k-1}[k,k])=A^{k-1}[i,k]$$
/// となることから(A[k,j]についても同様)、  
/// k-1->kの更新中にA[i,k],A[k,j]を上書きしても不都合はなく、A[i,j]は2次元で保持できる。
/// 
/// なお、A[i,i]<0のとき、負の閉路が存在することになる。
/// 
/// 経路復元に関しては記事は少ないが、[参考](http://zeosutt.hatenablog.com/entry/2015/05/05/045943)によると、
/// もとの隣接行列を用いてO(|V|^2)で復元できることから、アルゴリズムのO(|V|^3)よりも小さくて問題にならないのかも、とか。  
/// 一応、経路更新時に前の点や後の点を更新する別の2次元行列を用意することで同時記録はできそう。
/// 
/// ## スニペット名
/// `snp-warshall-floyd`
/// 
/// ## 利用例
/// [AOJ GRL1_C](https://onlinejudge.u-aizu.ac.jp/courses/library/5/GRL/1/GRL_1_C)を利用
/// ```rust
/// use my_rust_snip::graph::WarshallFloyd;
/// use std::i64;
/// 
/// let dist = vec![
///     vec![0, 1, 5, i64::MAX],
///     vec![i64::MAX, 0, 2, 4],
///     vec![i64::MAX, i64::MAX, 0, 1],
///     vec![i64::MAX, i64::MAX, 7, 0],
///     ];
/// 
/// let mut wf = WarshallFloyd::new(dist);
/// wf.solve();
/// assert!(!wf.has_negative_close());
/// 
/// let dist = vec![
///     vec![0, 1, 5, i64::MAX],
///     vec![i64::MAX, 0, 2, 4],
///     vec![i64::MAX, i64::MAX, 0, 1],
///     vec![i64::MAX, i64::MAX, -7, 0],
///     ];
/// let mut wf = WarshallFloyd::new(dist);
/// wf.solve();
/// assert!(wf.has_negative_close());
/// ```
/// 
#[snippet("snp-warshall-floyd")]
#[snippet(prefix="use std::{i64,cmp};")]
pub struct WarshallFloyd {
    n: usize,
    pub(crate) dist: Vec<Vec<i64>>,
}

#[snippet("snp-warshall-floyd")]
impl WarshallFloyd {
    /// 与えられた隣接行列を委譲し、構造体を構築
    /// 隣接行列はd[i,i]=0、非接続点はi64::MAXとしている
    pub fn new(dist: Vec<Vec<i64>>) -> WarshallFloyd {
        WarshallFloyd {
            n: dist.len(),
            dist: dist,
        }
    }

    /// 最短コストの計算を行う
    pub fn solve(&mut self) {
        for k in 0..self.n {
            for i in 0..self.n {
                for j in 0..self.n {
                    // MAXを含む加算はエラーになるため除外しておく
                    if self.dist[i][k] != i64::MAX && self.dist[k][j] != i64::MAX {
                        self.dist[i][j] = cmp::min(self.dist[i][j], self.dist[i][k] + self.dist[k][j]);
                    }
                }
            }
        }
    }

    /// AOJの仕様に合わせたPrint関数。適宜変更する
    pub fn print(&self) {
        if self.has_negative_close() {
            println!("NEGATIVE CYCLE");
        } else {
            for i in 0..self.n {
                let str = self.dist[i].iter()
                    .map(|v| { match v {
                        &i64::MAX => "INF".to_string(),
                        x => x.to_string(),
                    }})
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", str);
            }
        }
    }

    /// 負の閉路を持っているか判定する
    pub fn has_negative_close(&self) -> bool {
        for i in 0..self.n {
            if self.dist[i][i] < 0 {
                return true;
            }
        }
        return false;
    }
}


#[cfg(test)]
mod test {
    use super::WarshallFloyd;
    use std::i64;

    #[test]
    fn test_warshallfloyd(){
        let dist = vec![
            vec![0, 1, 5, i64::MAX],
            vec![i64::MAX, 0, 2, 4],
            vec![i64::MAX, i64::MAX, 0, 1],
            vec![i64::MAX, i64::MAX, 7, 0],
            ];
        let ans = vec![
            vec![0, 1, 3, 4],
            vec![i64::MAX, 0, 2, 3],
            vec![i64::MAX, i64::MAX, 0, 1],
            vec![i64::MAX, i64::MAX, 7, 0],
            ];
    
        let mut wf = WarshallFloyd::new(dist);
        wf.solve();
        assert_eq!(wf.dist, ans);
        assert!(!wf.has_negative_close());
    }

    #[test]
    fn test_negative_cycle(){
        let dist = vec![
            vec![0, 1, 5, i64::MAX],
            vec![i64::MAX, 0, 2, 4],
            vec![i64::MAX, i64::MAX, 0, 1],
            vec![i64::MAX, i64::MAX, -7, 0],
            ];
        let mut wf = WarshallFloyd::new(dist);
        wf.solve();
        assert!(wf.has_negative_close());
    }
}