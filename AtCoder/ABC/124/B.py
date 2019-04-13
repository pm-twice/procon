# -*- coding: utf-8 -*-

N = int(input())
H = [int(h) for h in input().split()]

cnt = 1
mx = H[0]

for h in H[1:]:
    if h >= mx:
        cnt+=1
        mx = h

print(cnt)
