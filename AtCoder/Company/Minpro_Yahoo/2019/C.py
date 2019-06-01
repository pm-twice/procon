# -*- coding: utf-8 -*-

K, A, B = map(int, input().split())

if K == 1:
    print(2)
elif B - A < 3:
    # 2回かけるので3以上増えないと意味ない
    print(1+K)
else:
    # b < Aまでは+1で増やす (A-1回)
    # b >= Aなら2回で-A+B増やす

    # -A+Bを行える回数を調べる
    d = K - A + 1
    if d % 2 == 0:
        res = A + (d//2)*(B-A)
    else:
        res = A+1 + (d//2)*(B-A)

    print(res)


# 以下はK=10^9に対応できない
# bbefore = 1 # K = 0
# before = 2  # K = 1
#
# # yenはビスケット→円、円→ビスケットで使い切るので計算不要
# for i in range(2, K+1):
#     # increase biscuit
#     b1 = before + 1
#     # exchange
#     if bbefore >= A:
#         b2 = (bbefore - A) + B
#     else:
#         b2 = 0
#     bbefore = before
#     before = max(b1, b2)
#
# print(before)
