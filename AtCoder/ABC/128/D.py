#-*- coding:utf-8 -*-
from collections import deque

N, K = map(int, input().split())
V = [int(_) for _ in input().split()]

# 4^100 = 1e60 より全探索は不可
# 左右操作というところで詰まる。片側ならどうにかなるんだが…

# dp[k]: K回取りだしてk個残してb個戻す: K=k+2*b (k>b)

dp = [0 for _ in range(K)]

for b in range(0, K):
    k = K - 2*b
    if k < b:
        continue
    else:
        pass


# 2次元dp？
# dp[k]: (v, que, have)
# k: j+1回の操作時の最大値

# dp = [None for k in range(K+1)]


# def rec(dp, k):
#     if k == 0:  # 操作0回
#         dp[0] = (0, deque(V), [])
#     elif k == 1:   # 操作1回
#         q = deque(V)
#         mx = max(q[0], q[-1], 0)
#         hv = []
#         if q[0] == mx:
#             hv.append(q.popleft())
#         elif q[-1] == mx:
#             hv.append(q.pop())
#         dp[1] = (mx, q, hv)
#     else:
#         # hv: 昇順ソート済みとする
#         # dp[k-1]から1回操作の最大値
#         # dp[k-2]から2回操作の最大値 (左右同じところ)()

#         # 1回操作
#         q = deque(dp[k-1][1])
#         mx = max(q[0], q[-1], 0)
#         hv = list(dp[k-1][2])
#         if q[0] == mx:
#             hv.append(q.popleft())
#         elif q[-1] == mx:
#             hv.append(q.pop())
#         dp[k] = (dp[k-1][0]+mx, q, hv)



