# -*- coding: utf-8 -*-

S = input()
mx = 0

for i in range(len(S)):
    cnt = 0
    j = i
    while j < len(S) and (S[j] == "A" or S[j] == "T" or S[j] == "C" or S[j] == "G"):
        cnt += 1
        j += 1
    if cnt > mx:
        mx = cnt

print(mx)
