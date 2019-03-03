# -*- coding: utf-8 -*-
# D_ans.pyからコメントなど不要部分削除したもの
import sys
sys.setrecursionlimit(10**6) # rootの結合の再帰がテストケースによっては深いので、指定が必要

class UnionFindbySize:
    def __init__(self, n):
        self.parent = [-1 for i in range(n)]

    def root(self, x):
        if self.parent[x] < 0:
            return x
        else:
            # 再帰深くなるので要注意。場合によってはqueとかで実装した方がいいかも
            self.parent[x] = self.root(self.parent[x])
            return self.parent[x]

    def issame(self, x, y):
        return self.root(x) == self.root(y)

    def unite(self, x, y):
        x = self.root(x)
        y = self.root(y)
        if x == y:
            return False
        else:
            self.parent[y] += self.parent[x]    # yの要素数を増やす
            self.parent[x] = y
            return True

    def size(self, x):
        return -self.parent[self.root(x)]

N, M = map(int, input().split())
A = [0] * M
B = [0] * M
for i in range(M):
    A[i], B[i] = map(int, input().split())
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
