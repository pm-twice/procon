# -*- coding: utf-8 -*-
# import math   # math.infが使えない？(3.5から。AtCoderは3.4)

# 解説見ながら記述
# 延長魔法は、合成前・合成後で実施しても等価。
# 合成魔法が使えないパターンにのみ延長魔法を使うようにして全探索するみたい

N, A, B, C = map(int, input().split())
ls = [int(input()) for i in range(N)]
INF = 10 ** 10  # int32.max = 2*10^9より大きくしている

def rec(cur, a, b, c):
    """
    cur: 現在のlsのindex
    a: 合成中の1本目の長さ
    b: 合成中の1本目の長さ
    c: 合成中の1本目の長さ
    """
    if cur == N:
        if min(a, b, c) > 0:
            # 合成1回目：1本目は選ぶだけで合成する必要ないので10*3を引いておく
            return abs(A-a) + abs(B-b) + abs(C-c) - 30
        else:
            # 1回も合成していない場合は無効
            # return math.inf
            return INF
    else:
        r0 = rec(cur+1, a, b, c)    # ls[cur]に対して何もしない
        r1 = rec(cur+1, a+ls[cur], b, c) + 10    # ls[cur]をaに合成
        r2 = rec(cur+1, a, b+ls[cur], c) + 10    # ls[cur]をbに合成
        r3 = rec(cur+1, a, b, c+ls[cur]) + 10    # ls[cur]をcに合成
        return min(r0, r1, r2, r3)

r = rec(0, 0, 0, 0)

print(r)
