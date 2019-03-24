# -*- coding: utf-8 -*-

N, Q = map(int, input().split())
S = input()

# そのままカウントしたのでは、O(Q(r-l))の時間がかかる
# Sと同じ長さ(<10^5)の配列ACを作って、Sの添え字に対応するAC数をカウントし、
# AC[r]-AC[l]で差分を取れば求める部分文字列のACはカウントできそう。
# これでO(N)で終わるはず

# count AC
AC = [0] * N
for i in range(1,N):
    if S[i-1] == "A" and S[i] == "C":
        AC[i] = AC[i-1] + 1
    else:
        AC[i] = AC[i-1]

# answer
for i in range(Q):
    l, r = map(int, input().split())
    print(AC[r-1]-AC[l-1])
