#-*- coding:utf-8 -*-

S = input()
s1 = S[:2]
s2 = S[2:]
i1 = int(s1)
i2 = int(s2)

YYMM = False
MMYY = False

if 1 <= i1 <= 12:
    MMYY = True
if 1 <= i2 <= 12:
    YYMM = True

if YYMM and MMYY:
    print("AMBIGUOUS")
elif YYMM:
    print("YYMM")
elif MMYY:
    print("MMYY")
else:
    print("NA")


