#-*- coding:utf-8 -*-

A, B = map(int, input().split())

if A <= 5:
    r = 0
elif A < 13:
    r = B//2
else:
    r = B

print(r)