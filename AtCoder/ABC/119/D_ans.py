# -*- coding: utf-8 -*-
import bisect

A, B, Q = map(int, input().split())
INF = 10**18
S = [-INF, -INF] + [int(input()) for i in range(A)] + [INF, INF]
T = [-INF, -INF] + [int(input()) for i in range(B)] + [INF, INF]

# xから見た際、最寄りの寺と神社を1件ずつ訪れる方法は以下のパターンが考えられる
# 1. 神社(正方向)→寺(正方向)
# 2. 神社(負方向)→寺(正方向)
# 3. 神社(正方向)→寺(負方向)
# 4. 神社(負方向)→寺(負方向)
# 5. 寺(正方向)→神社(正方向)
# 6. 寺(負方向)→神社(正方向)
# 7. 寺(正方向)→神社(負方向)
# 8. 寺(負方向)→神社(負方向)

# 1895msでギリギリ。

d = [INF] * 8
for q in range(Q):
    x = int(input())
    # S,Tはソート済み
    # 二部探索で最寄りの神社を探す: bisectを用いる
    sp = bisect.bisect_left(S, x) # 正方向最寄神社のindex
    tp = bisect.bisect_left(T, S[sp]) # 正方向最寄寺のindex
    tn = tp - 1 # 負方向最寄寺のindex
    d[0] = abs(S[sp] - x) +  abs(S[sp]-T[tp])
    d[1] = abs(S[sp] - x) +  abs(S[sp]-T[tn])

    sn = sp - 1 # 負方向最寄神社のindex
    tp = bisect.bisect_left(T, S[sn]) # 正方向最寄寺のindex
    tn = tp - 1 # 負方向最寄寺のindex
    d[2] = abs(S[sn] - x) +  abs(S[sn]-T[tp])
    d[3] = abs(S[sn] - x) +  abs(S[sn]-T[tn])

    # 最寄りの寺を探す
    tp = bisect.bisect_left(T, x) # 正方向最寄寺のindex
    sp = bisect.bisect_left(S, T[tp]) # 正方向最寄神社のindex
    sn = sp - 1 # 負方向最寄神社のindex
    d[4] = abs(T[tp] - x) +  abs(T[tp]-S[sp])
    d[5] = abs(T[tp] - x) +  abs(T[tp]-S[sn])

    tn = tp - 1 # 負方向最寄寺のindex
    sp = bisect.bisect_left(S, T[tn]) # 正方向最寄神社のindex
    sn = sp - 1 # 負方向最寄神社のindex
    d[6] = abs(T[tn] - x) +  abs(T[tn]-S[sp])
    d[7] = abs(T[tn] - x) +  abs(T[tn]-S[sn])
    print(min(d))
