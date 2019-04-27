# -*- coding: utf-8 -*-

def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a%b)

N = int(input())
A = [int(a) for a in input().split()]

# GCDの結合則を利用する
# 左端、右端からの累積GCDを求めておくと、Aiを取り除いた場合のGCDがすぐに計算できる。
# つまり、
# L(i): n=0,...,iまでのGCD 
# R(i): n=i,...,N-1までのGCD
# このとき、Aiを除いた全体のGCDは、GCD(L(i-1), R(i+1))で計算できる。

L = [0] * N
R = [0] * N

L[0] = A[0]
R[-1] = A[-1]
for i in range(1, N):
    L[i] = gcd(L[i-1], A[i])
    idx = N-1-i
    R[idx] = gcd(R[idx+1], A[idx])

mx = 0
for i in range(N):
    if i == N-1:
        v = L[N-2]
    elif i == 0:
        v = R[1]
    else:
        v = gcd(R[i+1], L[i-1])
    mx = max(v, mx)
print(mx)