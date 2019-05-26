#-*- coding:utf-8 -*-

N, M = map(int, input().split())
sw = [[int(_)-1 for _ in input().split()] for i in range(M)]
p = [int(_) for _ in input().split()]

def rec(n, onoff, lmp):
    if n < N:
        onoff[n] = True
        s = rec(n+1, onoff, lmp)
        onoff[n] = False
        s += rec(n+1, onoff, lmp)
        return s
    else:
        sm = 0
        for i in range(M):
            r = 0
            for j in sw[i][1:]:
                if onoff[j]:
                    r+=1
            if r % 2 == lmp[i]:
                sm += 1
        if sm == M:
            return 1
        else:
            return 0

onoff = [False for _ in range(N)]
print(rec(0, onoff, p))