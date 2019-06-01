# -*- coding: utf-8 -*-

S = input()
k = len(S)
wn = len([s for s in S if s == "o"])

if wn+15-k >= 8:
    print("YES")
else:
    print("NO")