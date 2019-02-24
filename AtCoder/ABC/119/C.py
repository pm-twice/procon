# -*- coding: utf-8 -*-

# A,B,Cのいずれかがliと差異10以内だと、延長・短縮でOK
# liと10より大きい場合は合成する方が得
# lは1000以内なので、全列挙となるのか？
# バックトラックありうることを考えると、再帰がよさそう？

def func(val, ls, mp):
    if val not in ls:
        mn1 = [(i, abs(val-l)) for i, l in enumerate(ls)]   # 延長・縮小の場合
        mn2 = []   # 合成全パターン
        for i1, l1 in enumerate(ls):
            for i2, l2 in enumerate(ls[1:], start=i1+1):
                mn2.append((i1, i2, abs(l1+l2-val)))
        print(mn1)
        print(mn2)
        m1 = min(mn1, key=(lambda x: x[1]))
        m2 = min(mn2, key=(lambda x: x[2]))
        if  m1[1] < m2[2] + 10:
            mp += m1[1]
            ls[m1[0]] = val
        else:
            mp += 10
            v1, v2 = ls[m2[0]], ls[m2[1]]
            ls.remove(v1)
            ls.remove(v2)
            ls.append(v1+v2)
            mp = func(val, ls, mp)
        ls.sort()
    else:
        pass
    return mp

N, A, B, C = map(int, input().split())
ls = [0] * N
for i in range(N):
    ls[i] = int(input())

# 昇順にソート
ls.sort()

mp = 0

mnA = min([abs(l-A) for l in ls])
mnB = min([abs(l-B) for l in ls])
mnC = min([abs(l-C) for l in ls])
if mnC <= mnA and mnC <= mnB:
    mp = func(C, ls, mp)
elif mnB <= mnA and mnB <= mnC:
    mp = func(B, ls, mp)
    B = C
else:
    mp = func(A, ls, mp)
    A = C

mnA = min([abs(l-A) for l in ls])
mnB = min([abs(l-B) for l in ls])
if mnA <= mnB:
    mp = func(A, ls, mp)
    A = B
else:
    mp = func(B, ls, mp)

mp = func(A, ls, mp)




print(mp)


#
# def rec(ls, A, B, C, mp):
#     """ C→B→Aの順にNoneとなるものとする"""
#     if A is None and B is None and C is None:
#         print(mp)
#     elif B is None and C is None:
#     elif C is None:
#     else:
#         # A, B, Cのうち、最も移動距離が少ないものを探す
#         mnA = [i, abs(l-A) for i, l in enumerate(ls)]
#         mnB = [i, abs(l-B) for i, l in enumerate(ls)]
#         mnC = [i, abs(l-C) for i, l in enumerate(ls)]
#         mnA.sort(key=(lambda x: x[1]))
#         mnB.sort(key=(lambda x: x[1]))
#         mnC.sort(key=(lambda x: x[1]))
#
#         if mnA[0][1] < mnB[0][1] and mnA[0][1] < mnC[0][1]:
#             if mnA[0][1] <= 10:
#                 ls[mnA[0][0]] = A
#                 mp += mnA[0][1]
#                 ls.sort()
#                 rec(ls, B, C, None, mp)
#             else:
#                 ds = ls[0] + ls[1]
#                 if ls[mnAid] > A:
#                     ls[mnAid] = A
#                     mp += mnA
#                     rec(ls, B, C, None, mp)
