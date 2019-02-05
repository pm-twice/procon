# -*- coding: utf-8 -*-

N = int(input())
L = [int(v) for v in input().split()]
lm = max(L)
sl = sum(L) - lm
if sl > lm:
    print("Yes")
else:
    print("No")
