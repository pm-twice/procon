# -*- coding: utf-8 -*-

N = int(input())
X, U = [], []
sm = 0.0
RATE = 380000.0

for i in range(N):
    x, u = input().split()
    if u == "JPY":
        x = int(x)
        sm += x
    elif u == "BTC":
        x = float(x)
        sm += RATE * x
    else:
        raise ValueError("")

    X.append(x)
    U.append(u)

print(sm)
