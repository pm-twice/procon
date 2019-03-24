# -*- coding: utf-8 -*-

N = int(input())

# 4^100 ≒1.61*10^60
# 全探索はまず不可能。N=3の場合から漸化式？いまいち規則性が分からん

# N文字の場合、ATCGの4通りで4**N通りの文字列
# 1. この時、"AGC"を含まない
# 2. 隣接2文字の入れ替えで"AGC"にならない
#   つまり、"GAC", "ACG"を含むのもアウト
#   "AxGC", "AGxC"もアウト
# N-1における上記を満たす文字数をf(N-1)とすると
# 先頭がGC, AC, CGの場合にA,G,Aを置くのがアウト
# また、先頭がxGCの場合(TGC, GGC)もAを置くのはアウト
# (CGCの場合は、CGの場合に内包される)
# また、先頭がGxCの場合(GGC, GTC)もAを置くのはアウト
# (GAC,GCCは既に除外済み)
# さらにこれを10^9+7で割った余りを返す必要がある
# __わからん__

# 先頭4文字に依存 (256通り)
# 除外済み: AGCx, xAGC, AxGC, AGxC, GACx, ACGx, xGAC, xACG (AAGC,AGCC,AGAC,GACGが被り)
# →4*8-4=28通りが除外→228? (正解は230)

# n=3: 61 (4**3 - 3)
# n=4: 230 (4**4 - 26)
#
# def dfs(N):
#     if N == 1:
#         return ["A", "G", "T", "C"]
#     else:
#         r = []
#         for c in dfs(N-1):
#             r.append("A"+c)
#             r.append("G"+c)
#             r.append("T"+c)
#             r.append("C"+c)
#         return r
#
# r = dfs(N)
# r2 = []
# for c in r:
#     if "AGC" in c:
#         continue
#     elif "GAC" in c:
#         continue
#     elif "ACG" in c:
#         continue
#     elif "AGGC" in c:
#         continue
#     elif "ATGC" in c:
#         continue
#     elif "AGTC" in c:
#         continue
#     else:
#         r2.append(c)
#
# print(len(r2))



def rec(n):
    if n == 3:
        # (AGC, GAC, ACG)がアウト。4**3-3=61
        return 61
    else:
        # Tを先頭に置く場合: rec(n-1)通り
        # Cを先頭に置く場合: rec(n-1)通り
        # Gを先頭に置く場合: (GACを避ける)
        # Aを先頭に置く場合: (AGCx, ACGx, AGGC, ATGC, AGTCを避ける)
        return rec(n-1)*4 - 14  # N=4に無理やり合わせてもダメ


r = rec(N)
r = r % (10**9+7)

print(r)
