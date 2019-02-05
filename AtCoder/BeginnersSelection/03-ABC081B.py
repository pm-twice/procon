# -*- coding: utf-8 -*-
# Shift Only
n = int(input())
A = list(map(int, input().split()))
c = 0
while True:
    if any([a % 2 == 1 for a in A]):    # any Odd
        break
    else:   # all Even
        c += 1
        A = [a // 2 for a in A]
print(c)
