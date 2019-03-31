# -*- coding: utf-8 -*-

N, M, C = map(int, input().split())
B = [int(b) for b in input().split()]
res = 0
for i in range(N):
    A = [int(a) for a in input().split()]
    s = sum([a*b for a, b in zip(A, B)])+C
    if s > 0:
        res += 1
print(res)
