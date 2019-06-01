# -*- coding: utf-8 -*-

# 2分探索使って左から落ちるもののうち最も右にあるやつ、
# 右から落ちるもので最も左にあるやつを探索
# めぐる式二分探索を使ってみている。
# https://qiita.com/drken/items/97e37dd6143e33a64c8c

from collections import deque

N, Q = map(int, input().split())
s = input()
TD = [input().split() for i in range(Q)]

def simulate(n):
    # n番目のマスが呪文完了後に右から落ちるなら1、左から落ちるなら-1、
    # そうでないなら0を返す関数
    if n < 0:
        return -1
    if n >= N:
        return 1
    for t, d in TD:
        if t == s[n]:
            if d == "L":
                n-=1
            else:
                n+=1
        if n < 0:
            return -1
        if n >= N:
            return 1
    return 0


# 左に落ちるもの
l, r = -1, N
while l+1 < r:
    n = (l+r)//2
    sim = simulate(n)
    if sim == -1:
        l = n
    else:
        r = n
L = l

# 右に落ちるもの
l, r = -1, N
while l+1 < r:
    n = (l+r)//2
    sim = simulate(n)
    if sim == 1:
        r = n
    else:
        l = n
R = r

print(R-L-1)
