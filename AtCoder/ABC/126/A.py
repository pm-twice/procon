# -*- coding: utf-8 -*-

N, K = map(int, input().split())
S = input()
l = list(S)
l[K-1] = l[K-1].lower()
R = "".join(l)

print(R)


