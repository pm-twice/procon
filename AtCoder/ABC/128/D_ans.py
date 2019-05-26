#-*- coding:utf-8 -*-
from collections import deque

N, K = map(int, input().split())
V = [int(_) for _ in input().split()]

mn = min(N,K)   # 取り出せる個数
ans = 0
for a in range(mn+1):
    for b in range(mn+1-a):
        que = deque(V)
        have = []
        for i in range(a):  # 左からの取り出し
            have.append(que.popleft())
        for i in range(b):  # 右からの取り出し
            have.append(que.pop())
        have.sort() # 昇順ソート
        c = K - a - b   # c回まで除去可能
        neg = [v for v in have if v < 0]
        if len(neg) < c:
            ans = max(ans, sum(have[len(neg):]))
        else:
            ans = max(ans, sum(have[c:]))

print(ans)