#-*- coding:utf-8 -*-

import math

N, K = map(int, input().split())

# 1. サイコロを振って得点にする
# 2. 得点がKより小さいならコインを振る
#   1. コインが裏なら負け
#   2. コインが表なら得点は2倍に
# 出す目それぞれについて、Kを超えるのに必要な2倍の数を求めて足し合わせる

# a2^n > K ⇔ nlog2 +loga > logK ⇔ n > (logK-loga) / log2
# aについて、勝利できる確率はn回表を出す=1/2^n
# あとはaについて1/Nを掛けて足し合わせればOK

p = 0.0

for i in range(1, N+1):
    if K > i:
        n = math.log(K/i) / math.log(2)
        n = math.ceil(n)
        p += (1/2**n) / N
    else:
        p += 1/N

print(p)





