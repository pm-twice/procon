# -*- coding: utf-8 -*-
# Some Sums

n, a, b = map(int, input().split())
c = 0
for i in range(1, n+1):
    s = 0
    t = i
    while True:
        s += t % 10
        t = t // 10
        if t <= 0:
            break
    if a <= s <= b:
        c += i
print(c)
