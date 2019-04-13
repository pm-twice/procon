# -*- coding: utf-8 -*-

N, K = map(int, input().split())
S = input()
bs = int(S,2)
ls = list(S)

# N, Kがそれぞれ10^5以下なので、O(NK)は不可能
# 1の連続Bit数を最大化する

def cnt_max_ones(x):
    """x:intのうち、連続する1のビットで最大長のものを返す"""
    mx = 0
    cnt = 0
    while x > 0:
        if x & 1 == 1:
            cnt += 1
        else:
            cnt = 0
        if cnt > mx:
            mx = cnt
        x >>= 1
    return mx

# 1100011などがあった時、000の長さ+左右の11の長さの合計が長い000を反転すればいい

# 結局のところ、一番長い000を反転すればよい？
# 1を含む区間を反転→反転で2回行うのであれば、その左右をそれぞれ反転した方がいい？
# さらに多くの複数区間をまたがる場合はどうなるんだ？

# とりあえず0の位置をカウントしてみる
zeros = []  # (start, end, len)
i = 0

while i < N:
    while i < N and S[i] == "1":
        i+=1
    if i >= N:
        break
    start = i
    end = i
    while i < N and S[i] == "0":
        end += 1
        i+=1
    zeros.append((start, end, end-start))

zeros.sort(key=(lambda x:x[2]), reverse=True)

print(S)
print(zeros)

# 0区間が長いものから反転していってみる

for start, end, ln in zeros[:K]:
    ls[start:end] = ["1" for i in range(ln)]

print(ls)

# テストケースに通らない。
# 解法はもっと複雑みたい

b = int("".join(ls),2)
print(cnt_max_ones(b))
