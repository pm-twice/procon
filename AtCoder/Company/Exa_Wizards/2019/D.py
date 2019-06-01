# -*- coding: utf-8 -*-

N, X = input().split()
S = [int(s) for s in input().split()]

class elem:
    def __init__(self, x, rest, cnt):
        self.x = x
        self.rest = rest
        self.cnt = cnt

dp = []
dp.append([None] * N)
for i in range(N):
    dp[0][i] = elem(x%s[i], [s for s in S if s != s[i]], 1)
for i in range(1, N):
    dp.append([None] * N)
    for j in range(N):
