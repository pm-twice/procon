# -*- coding: utf-8 -*-

A, B = map(int, input().split())

if B % A == 0:
    r = A+B
else:
    r = B-A
print(r)
