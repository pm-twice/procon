# -*- coding: utf-8 -*-

y, m, d = map(int, input().split("/"))

heisei = True
if y < 2019:
    heisei = True
elif y == 2019:
    if m < 4:
        heisei = True
    elif m == 4:
        if d <= 30:
            heisei = True
        else:
            heisei = False
    else:
        heisei = False
else:
    heisei = False

if heisei is True:
    print("Heisei")
else:
    print("TBD")
