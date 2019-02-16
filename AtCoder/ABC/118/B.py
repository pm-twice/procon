# -*- coding: utf-8 -*-

N, M = map(int, input().split())
cnt = [0]*(M+1)

for i in range(N):
    l = [int(i) for i in input().split()]
    for a in l[1:]:
        cnt[a] += 1
res = sum([1 for c in cnt if c == N])
print(res)
