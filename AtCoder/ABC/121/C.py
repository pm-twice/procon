# -*- coding: utf-8 -*-

N, M = map(int, input().split())
AB = [[int(x) for x in input().split()] for n in range(N)]

# O(NlogN)
AB.sort(key=(lambda x:x[0]))

# O(M)
cnt = 0 # counter of drink
prc = 0 # price of drink
i = 0
while cnt < M:
    a, b = AB[i]
    if cnt+b <= M:
        cnt += b
        prc += a*b
    else:
        prc += a*(M-cnt)
        cnt = M
    i+=1

print(prc)
