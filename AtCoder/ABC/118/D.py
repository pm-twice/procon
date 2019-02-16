# -*- coding: utf-8 -*-

N, M = map(int, input().split())
A = [int(i) for i in input().split()]

Nums = [2,5,5,4,5,6,3,7,6]  # 1,2,3,4.5,6,7,8,9を作るマッチの数
uses = [Nums[a-1] for a in A]   # 実際に使う本数

digits = [(a, N//Nums[a-1], N%Nums[a-1]) for a in A] # 全部Aiで作った際の商とあまり
print(digits)


rest = []
base = digits[0]

for i in range(1, M):
    if base[1] < digits[i][1]:
        rest.append(tuple(base))
        base = digits[i]
    elif base[1] == digits[i][1] and base[2] < digits[i][2]:
        rest.append(tuple(base))
        base = digits[i]
    else:
        rest.append(tuple(digits[i]))

print(rest)
print(base)

# d = sum(ni*ai) + (nm-x)*am
# d = (nm-2x)*am

d = base[2] # 開始時点のあまり
i = 0
while True:
    _d = d + i * base[1]    # 考察する余りの本数
    if is_creatable(_d, rest) is not None:
        # この余りの本数から、あまりなく整数を作れればそれを出力。

    else:
        i += 1


def is_creatable(num, rest):
    # ナップザック問題
    nums = [False] * num    # 1~numまでを作れるかどうか
    vals = [0] * nums   # 作れるとき、vals[i-vals[n]]

    # 時間切れ
    pass
