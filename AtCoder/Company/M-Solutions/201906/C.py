# -*- coding: utf-8 -*-

N, A, B, C = map(int, input().split())
a = A / 100
b = B / 100
c = C / 100

# K回の勝負の時、確率a,b,cを用いて、K回の勝負が行われる確率は
# PK = a^Na b^Nb c^Nc KCNa (K-Na)CNb
# K回で勝負が終了する確率は(N>Nb)
# P = a^(N-1) b^Nb c^Nc KC(N-1) (K-N+1)CNb a
#   + a^(Na) b^(N-1) c^Nc KC(N-1) (K-N+1)CNa b
#   = a^N b^Nb c^Nc KC(N-1) (K-N+1)CNb
#   + a^(Na) b^N c^Nc KC(N-1) (K-N+1)CNa
#   = c^Nc

# Aが勝つ場合: 挿入の場合の数を利用する
# 勝負がN回となる確率: a^N
# N+1: a^N N (b+c)  # N-1個を0個以上の2組に分ける: N個の隙間に1個挿入=N通り
# N+2: a^N N^2/2! (bb+2bc+cc)   # N-1個を0個以上の3組に分ける: N^2/2!
# N+3: a^N N^3/3! (b+c)^3       # N-1個を0個以上の4組に分ける: N^3/3!
# ...
# N+k: a^N N^k/k! (b+c)^k
# ※N^k/k! → 0 (k→∞)
#
# Aについての期待値
# Ak = sum_{i=0}^{k} (N+i) a^N N^i/i! (b+c)^i
#    = a^N sum_{i=0}^{k} (N+i) {N(b+c)}^i/(i!)
#    = a^N sum_{i=0}^{k} (N+i) {N(1-a)}^i/(i!)
#
# A,Bについての期待値
# Ek = sum_{i=0}^{k} (N+i)N^i/i! {a^N (1-a)^i + b^N (1-b)^i} 
# 解ける気がしない…








