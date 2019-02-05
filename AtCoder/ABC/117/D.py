# -*- coding: utf-8 -*-

N, K = map(int, input().split())
A = [int(v) for v in input().split()]

# K < 10^12より計算量爆発
# m = -1
# for x in range(K+1):
#     s = sum([x ^ a for a in A])
#     if m < s:
#         m = s
# print(m)

l = len(bin(K))
m = 0
for i in range(l):
    b = (1 << i)
    f = sum([b ^ a for a in A])
