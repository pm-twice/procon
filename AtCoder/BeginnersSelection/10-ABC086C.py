# -*- coding: utf-8 -*-
# Traveling


def travel(a1, a2):
    t1, x1, y1 = a1
    t2, x2, y2 = a2
    d = x2 - x1 + y2 - y1  # distance
    dt = t2 - t1  # time

    if dt < d:
        return False
    elif (dt - d) % 2 == 0:
        return True
    else:
        return False

N = int(input())
Acts = []
for i in range(N):
    Acts.append(list(map(int, input().split())))    # t, x, y

Acts = [[0, 0, 0]] + Acts   # add t0

if all([travel(a, b) for a, b in zip(Acts[:-1], Acts[1:])]):
    print("Yes")
else:
    print("No")

