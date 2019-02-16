# -*- coding: utf-8 -*-

N = int(input())
A = [int(i) for i in input().split()]

# min(A)より小さな値が作れるか検証していく感じか？
A.sort()
l = len(A)

tmp1 = list(A)
tmp2 = []

while len(tmp1) > 1:
    m = tmp1[0]
    tmp2.append(m)
    for d in tmp1:
        a = d % m
        if 0 < a < d:
            tmp2.append(a)
    tmp2.sort()
    tmp1 = tmp2
    tmp2 = []

print(tmp1[0])
