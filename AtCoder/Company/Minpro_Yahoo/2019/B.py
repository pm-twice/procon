# -*- coding: utf-8 -*-

a1, b1 = map(int, input().split())
a2, b2 = map(int, input().split())
a3, b3 = map(int, input().split())

cnt = [0] * 4
cnt[a1-1] += 1
cnt[b1-1] += 1
cnt[a2-1] += 1
cnt[b2-1] += 1
cnt[a3-1] += 1
cnt[b3-1] += 1

flag = True
for c in cnt:
    if c == 0 or c > 2:
        flag = False
        break

if flag:
    print("YES")
else:
    print("NO")
