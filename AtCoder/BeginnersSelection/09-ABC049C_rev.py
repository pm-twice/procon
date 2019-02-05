# -*- coding:utf-8 -*-
# 白昼夢 / Daydream
# Greedy+後ろから版

divide = ["dream", "erase", "dreamer", "eraser"]
divide_r = [d[::-1] for d in divide]
S = input()
r = S[::-1]

find = False
i = 0
while True:
    if i == len(r):
        find = True
        break
    in_find = False
    for d in divide_r:
        if r[i:].startswith(d):
            i += len(d)
            in_find = True
            break
    if in_find is False:
        break

if find is True:
    print("YES")
else:
    print("NO")
