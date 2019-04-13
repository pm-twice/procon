# -*- coding: utf-8 -*-

def bc(x):
    return bin(x).count("1")

S = input()
N = len(S)

# 0101...か1010...に変更するときの塗り替え数を見ればいい=XOR
B = "".join(["0" if i%2==0 else "1" for i in range(N)])
R = int(S, 2) ^ int(B, 2)
C = bc(R)
r = min(C, N-C) # N-Cが1010...の場合に対応
print(r)
