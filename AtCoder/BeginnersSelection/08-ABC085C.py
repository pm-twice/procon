# -*- coding: utf-8 -*-
# Otoshidama

N, Y = map(int, input().split())
find = False
for i in range(0, N+1):             # 1000 bill
    for j in range(0, N+1-i):       # 5000 bill
        # 10000 bill
        r = Y - i * 1000 - j * 5000
        k = r // 10000
        if r % 10000 == 0 and k + j + i == N:
            print("{} {} {}".format(k, j, i))
            find = True
            break
    if find is True:
        break

if find is False:
    print("-1 -1 -1")
