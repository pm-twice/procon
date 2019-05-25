#-*- coding:utf-8 -*-

N, M = map(int, input().split())
L, R = [None for _ in range(M)], [None for _ in range(M)]
for i in range(M):
    L[i], R[i] = map(int, input().split())

# L < R
st = -1
end = N
for i in range(M):
    if L[i] > st:
        st = L[i]
    if R[i] < end:
        end = R[i]

r = end - st + 1
if r < 0:
    r = 0
print(r)

