# -*- coding: utf-8 -*-
# Kagami Mochi

N = int(input())
D = []
for i in range(N):
    D.append(int(input()))
s = set(D)  # remove duplicate
n = len(s)
print(n)
