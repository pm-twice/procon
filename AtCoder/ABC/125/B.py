# -*- coding: utf-8 -*-

N = int(input())
V = [int(v) for v in input().split()]
C = [int(v) for v in input().split()]

mx = 0
for v, c in zip(V, C):
    if v > c:
        mx += v-c
print(mx)