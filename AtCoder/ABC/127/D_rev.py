#-*- coding:utf-8 -*-

import bisect

N, M = map(int, input().split())
A = [int(a) for a in input().split()]
BC = [0]*M
for i in range(M):
    BC[i] = [int(n) for n in  input().split()]

A.sort()    # 昇順ソートO(NlogN)
BC.sort(key=(lambda x: x[1]), reverse=True)   # Ciの値で降順ソート

st = 0 # 編集開始位置
for i in range(M):  # O(MlogN)
    b, c = BC[i]
    idx = bisect.bisect_left(A, c, st)  # A全体でのidxを返す
    edit = idx - st # 編集個数
    edit = min(b, edit)
    for j in range(edit):
        A[st+j] = c
    st += edit

print(sum(A))