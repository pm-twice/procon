# -*- coding: utf-8 -*-
import sys
sys.setrecursionlimit(10**6) # rootの結合の再帰がテストケースによっては深いので、指定が必要

"""
グラフの辺の削除は難しい
辺が1つもない状態から追加していくことを考える

全部橋が落ちたとき(最終状態)、全ての組が渡れない=N C 2
これをAns[M]とする
ans[M] = N * (N-1) // 2


ans[i+1]を用いてans[i]を表現する
i+1->iで、A[i+1]とB[i+1]で繋がる

1. A[i+1]とB[i+1]がつながっていない
A[i+1]から訪問可能な島をN1、B[i+1]から訪問可能な島をN2とすると、
ans[i] = ans[i+1] - N1*N2

2. A[i+1]とB[i+1]がすでにつながっている
ans[i] = ans[i+1]

これをそのまま実装すると、N1,N2を求めるのに時間がかかってしまう。
そこで、Union-Findというデータ構造を用いる

"""

class UnionFind:
    """
    - n個の要素のグループ分けを管理する
    - 初め、n個のものはすべて別グループ
    - 次の2種類のクエリが存在
        + まとめる: グループの統合
        + 判定: 2つの要素が同じグループに属しているかを判定
    - 配列による実装と木構造による実装がある: 計算時間上、Treeによる実装が必要
    - 同じグループなら同じ木、全体では森となる
        + まとめる: 片方の木の根からもう片方に辺を張る
        + 判定: 2つの要素を上に辿って、根が同じかどうかを見ればよい
    - 効率化
        1. 上向きに辿って再帰的に値を調べる際に、調べたら辺を直接根につなぎなおす (経路圧縮)
        2. 木の高さを持っておき、低い方を高い方につなぐ (by Rank)
        + 両方の工夫でO(α(N))
        + 片方でO(logN)

    ここでは、経路圧縮で実装

    https://www.slideshare.net/chokudai/union-find-49066733
    """
    def __init__(self, n):
        # 要素iの親の添え字を保持する。自身を指すなら根の値となる
        self.parent = [i for i in range(n)]

    def root(self, x):
        # 木の根を求める
        if self.parent[x] == x:
            # 根の場合
            return x
        else:
            # 経路圧縮
            self.parent[x] = self.root(self.parent[x])
            return self.parent[x]

    def issame(self, x, y):
        # x, yが同じ集合に属するか否か
        return self.root(x) == self.root(y)

    def unite(self, x, y):
        # x, yの所属する集合を併合
        x = self.root(x)
        y = self.root(y)
        if x == y:
            # 所属が同じ
            return
        else:
            # xの親をyにして辺を貼る
            self.parent[x] = y

class UnionFindbySize:
    """
    さらに上記のUnion-Find木で、各木のサイズを保持できるようにする

    """
    def __init__(self, n):
        # 要素iの親の添え字を保持する。自身を指すなら負の値となる
        # 負の値の絶対値が、その木に含まれる要素数
        self.parent = [-1 for i in range(n)]

    def root(self, x):
        # 木の根を求める
        if self.parent[x] < 0:
            # 根の場合
            return x
        else:
            # 経路圧縮
            self.parent[x] = self.root(self.parent[x])
            return self.parent[x]

    def issame(self, x, y):
        # x, yが同じ集合に属するか否か
        return self.root(x) == self.root(y)

    def unite(self, x, y):
        # x, yの所属する集合を併合
        x = self.root(x)
        y = self.root(y)
        if x == y:
            # 所属が同じ
            return False
        else:
            # xの親をyにして辺を貼る
            self.parent[y] += self.parent[x]    # yの要素数を増やす
            self.parent[x] = y
            return True

    def size(self, x):
        # xが所属するグループの要素数を返す
        return -self.parent[self.root(x)]

"""
解答
"""

N, M = map(int, input().split())
A = [0] * M
B = [0] * M
for i in range(M):
    A[i], B[i] = map(int, input().split())
    # 添え字が1スタートなので1減らす
    A[i] -= 1
    B[i] -= 1

uf = UnionFindbySize(N)
res = [0] * (M)
res[-1] = N*(N-1)//2

i = 1
for a, b in zip(A[::-1], B[::-1]):
    id = M - 1 - i
    if id < 0:
        break
    if uf.issame(a, b):
        res[id] = res[id+1]
    else:
        sa = uf.size(a)
        sb = uf.size(b)
        res[id] = res[id+1] - sa*sb
        uf.unite(a, b)
    i+=1

for r in res:
    print(r)
