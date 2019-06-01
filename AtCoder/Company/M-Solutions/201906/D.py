# -*- coding: utf-8 -*-

N = int(input())
A,B = [0]*N, [0]*N
for i in range(N):
    A[i], B[i] = map(int, input().split())
    A[i] -= 1
    B[i] -= 1
C = [int(c) for c in input().split()]

