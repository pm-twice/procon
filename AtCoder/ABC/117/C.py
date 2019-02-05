# -*- coding: utf-8 -*-

N, M = map(int, input().split())
X = [int(v) for v in input().split()]
lx = len(X)

# N=10^5, M=10^5ならO(NM)は間に合わない

if N >= M:
    print(0)
else:
    # 動的計画法M個の地点を1~N個のコマで回ることを考える。

    X.sort()    # 昇順にソート
    coma = [0] * N
    # n=1
    diff = [x2-x1 for x2, x1 in zip(X[1:], X[:-1])]
    diff.sort() # 昇順にソート
    coma[0] = sum(diff)

    # n >= 2
    for i in range(1, N):
        coma[i] = coma[i-1] - diff[-1]
        del diff[-1]

    print(coma[-1])
