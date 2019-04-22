# -*- coding: utf-8 -*-

N = int(input())
A = [int(input()) for n in range(N)]
MOD = 998244353

# 再帰で全探索VerだとN=20くらいでTLEになってしまう
# 事前に全周計算しておいて、刈り取りやってみる: これもTLE
SM = sum(A)

def rec(arr, n, r, g, b):
    if SM-r <= r or SM-g <= g or SM-b <= b:
        # 現在の辺の長さが三角形成立以上に長くなっていたら打ち切り
        return 0
    elif n < len(arr):
        sm = rec(arr, n+1, r+arr[n], g, b)
        sm += rec(arr, n+1, r, g+arr[n], b)
        sm += rec(arr, n+1, r, g, b+arr[n])
        return sm
    else:
        if r+g > b and g+b > r and b+r > g:
            # 三角形が成立するなら1
            return 1
        else:
            return 0

res = rec(A, 0, 0, 0, 0)
res %= MOD
print(res)
