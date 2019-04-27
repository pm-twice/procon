# -*- coding: utf-8 -*-

N = int(input())
A = [int(a) for a in input().split()]

# かっこいい解法版
# Ai, Ai+1を反転して-Ai,-Ai+1であるが、これを繰り返すことで、
# -Ai, Ai+1, Ai+2,....,Aj-1, -Aj というように、
# i < jとなるi,jのみを反転させ、間はそのままという操作が可能。
# ここから、負の数が偶数個であれば、全てを正にすることができる
# 負の数が奇数個であれば、絶対値が一番小さい数を残して残りを正にすればよい

ng = [a for a in A if a < 0]    # 負の数の抽出
ab = [abs(a) for a in A]    # 絶対値の和

if len(ng) % 2 == 0:
    print(sum(ab))
else:
    mn = min(ab)
    print(sum(ab)-2*mn)

