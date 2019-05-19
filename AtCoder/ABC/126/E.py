#-*- coding:utf-8 -*-

import sys
sys.setrecursionlimit(10000)

class UnionFind:
    def __init__(self, length):
        # 自身を親として初期化 (-1)。正ならサイズを表す
        self.ary = [-1 for i in range(length)]
    
    def get_root(self, x):
        # rootのidxを求める
        if self.ary[x] < 0:
            return x
        else:
            self.ary[x] = self.get_root(self.ary[x])
            return self.ary[x]

    def is_same(self, x, y):
        return self.get_root(x) == self.get_root(y)

    def unite(self, x, y):
        x = self.get_root(x)
        y = self.get_root(y)
        if x==y:
            return False
        else:
            if self.ary[x] > self.ary[y]:
                x,y = y,x
            self.ary[x] += self.ary[y]
            self.ary[y] = x
            return True


N, M = map(int, input().split())
X, Y, Z = [0]*M ,[0]*M, [0]*M
uf = UnionFind(N)

for i in range(M):
    X[i],Y[i],Z[i] = map(int, input().split())
    X[i]-=1
    Y[i]-=1

for i in range(M):
    uf.unite(X[i], Y[i])

# rootの数を数える＝グラフの数を数える
s = set()
for i in range(N):
    s.add(uf.get_root(i))

print(len(s))
