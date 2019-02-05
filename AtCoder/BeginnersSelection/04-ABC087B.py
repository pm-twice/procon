# -*- coding: utf-8 -*-
# Coins
a = int(input())
b = int(input())
c = int(input())
x = int(input())

p = 0   # combi num
for i in range(a+1):    # num of 500
    for j in range(b + 1):  # num of 100
        for k in range(c + 1):  # num of 50
            s = i * 500 + j * 100 + k * 50
            if s == x:
                p += 1
            elif s > x:
                break
print(p)
