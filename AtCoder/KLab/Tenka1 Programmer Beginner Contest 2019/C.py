# -*- coding: utf-8 -*-

N = int(input())
S = input()

# 全探索だと2^Nで無理

# WWWW...BBBB...となるように変える。(全てW、全てBもOK)
# W: .
# B: #

W = [0] * N
B = [0] * N

if S[0] == ".":
    W[0] = 1
else:
    B[0] = 1

for i in range(1, N):
    if S[i] == ".":
        W[i] = W[i-1]+1
        B[i] = B[i-1]
    else:
        W[i] = W[i-1]
        B[i] = B[i-1]+1

# i-1までをWに、iからをBに変えるとする
# W[i-1]で、i-1までのWの個数が分かる(Wi-1)
# この時、i-(Wi-1)個のBをWに変える必要がある
# W[-1] - W[i-1]で、Bに変える必要のあるWの個数が分かる

mn = min(W[-1], B[-1]) # 全部B, 全部Wに変える場合を計算
for i in range(1, N):
    toW = i - W[i-1]
    toB = W[-1] - W[i-1]
    mn = min(mn, toB+toW)
print(mn)
