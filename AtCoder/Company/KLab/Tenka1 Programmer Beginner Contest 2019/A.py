# -*- coding: utf-8 -*-

A, B, C = map(int, input().split())

if B-A > 0 and A < C and C < B:
    print("Yes")
elif B-A < 0 and A > C and C > B:
    print("Yes")
else:
    print("No")
