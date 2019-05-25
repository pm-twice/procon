#-*- coding:utf-8 -*-

import bisect

N, M = map(int, input().split())
A = [int(a) for a in input().split()]
BC = [0]*M
for i in range(M):
    BC[i] = [int(n) for n in  input().split()]

A.sort()    # 昇順ソートO(NlogN)
BC.sort(key=(lambda x: x[1]), reverse=True)   # Ciの値で降順ソート

for i in range(M):
    b, c = BC[i]
    # cが降順なので、入替回数が減る？→TLE変わらず
    # 2部探索木で入れ替え、回転とかやる必要あり？
    idx = bisect.bisect_left(A, c)
    if idx > 0:
        for j in range(min(idx,b)):
            A[j] = c
        A.sort()

print(sum(A))