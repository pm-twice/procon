# -*- coding: utf-8 -*-

S = input()
cnt = 0

# O(N^2)は不可。
# 010がある時、01と10を取り除くパターンを作る？

# 時間超過
s = S
while True:
    c1, c2 = 0, 0
    ls = len(s)
    for i in range(ls-1):
        if s[i] == "0" and s[i+1] == "1":
            c1 += 1
        elif s[i] == "1" and s[i+1] == "0":
            c2 += 1
    if c1 == 0 and c2 == 0:
        break
    elif c1 >= c2:
        s = s.replace("01", "")
        cnt += c1*2
    else:
        s = s.replace("10", "")
        cnt += c2*2

print(cnt)


# 時間超過
# s = S
# while True:
#     ls = len(s)
#     s1 = s.replace("01", "")
#     s2 = s.replace("10", "")
#     ls1 = len(s1)
#     ls2 = len(s2)
#     if ls == ls1 and ls == ls2:
#         break
#     elif ls1 <= ls2:
#         cnt += (ls - ls1)
#         s = s1
#     else:
#         cnt += (ls - ls2)
#         s = s2
#
# print(cnt)

# 時間超過
# s = set()
# s.add(S)
# while True:
#     #print(s)
#     s2 = set()
#     for d in s:
#         for i in range(len(d)-1):
#             if d[i] != d[i+1]:
#                 ns = None
#                 if i+2 < len(d):
#                     ns = d[:i]+d[i+2:]
#                 else:
#                     ns = d[:i]
#                 if ns is not None and len(ns) > 0:
#                     s2.add(ns)
#     if len(s2) > 0:
#         cnt += 2
#         s = s2
#     else:
#         break
# print(cnt)
