# -*- coding: utf-8 -*-

# 全列挙版？
# 合成の順番についてはどうすればいいんだ…？

N, A, B, C = map(int, input().split())
ls = [0] * N
for i in range(N):
    ls[i] = int(input())

dp = [10000]*1000
for v in ls:
    for i, d in enumerate(dp):
        if abs(i+1-v) < d:
            dp[i] = abs(i+1-v)

for

print(dp)
