# -*- coding: utf-8 -*-

A, B = map(int, input().split())

res = max([A+B,A+A-1,B+B-1])

print(res)
