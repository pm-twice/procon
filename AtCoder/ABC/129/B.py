#-*- coding:utf-8 -*-

N = int(input())
W = [int(_) for _ in input().split()]

sm = sum(W)
acc = [0]*N
s = 0
for i in range(N):
    acc[i] = s + W[i]
    s += W[i]

mn = 100
for i in range(N):
    s = abs(acc[i] - (sm-acc[i]))
    mn = min(mn, s)

print(mn)
