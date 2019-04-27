# -*- coding: utf-8 -*-

def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a%b)

N = int(input())
A = [int(a) for a in input().split()]

if N <= 2:
    print(max(A))
    exit()

gc = [0] * (N-1)
for i in range(0,N-1):
    gc[i] = gcd(A[i],A[i+1])
#print(gc)
# このgcで最小のものが算出されるところを書き換えればよい？
# 連続してれば共通部、連続してないなら端とか？

mnd = -1
mn = 1e10
for i in range(0, N-1):
    if mn >= gc[i]:
        mnd = i
        mn = gc[i]

if mnd == N-1 and gc[-1] < gc[-2]:
    mnd = N-1

# 連続してれば後者になるのでmndはA[i]に対応することになる
# gcの長さはN-1

B = [a for a in A if a != A[mnd]]
#print(B)

lb = len(B)
if lb == 0:
    g = A[mnd]
elif lb >= 2:
    g = B[0]
    for i in range(1, len(B)):
        g = gcd(g, B[i])
else:
    g = B[0]
print(g)
