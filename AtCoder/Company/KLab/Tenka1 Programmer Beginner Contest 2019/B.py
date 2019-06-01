# -*- coding: utf-8 -*-

N = int(input())
S = input()
K = int(input())

s = S[K-1]

R = "".join([c if c == s else "*" for c in S])

print(R)
