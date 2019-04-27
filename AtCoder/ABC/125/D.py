# -*- coding: utf-8 -*-

N = int(input())
A = [int(a) for a in input().split()]
# B = [a for a in A]

# 正の数p, 負の数nに対して、p+n=N
# 全探索は2^Nで爆発

# sm2 = [A[i] + A[i+1] for i in range(0, N-1)]
# sm3 = [A[i] + 2*A[i+1] + A[i+2] for i in range(0, N-2)]

# 最終状態から考える
# A[i]の符号は以下の時に正、
# 1. A[i-1]+A[i] > 0
# 2. A[i+1]+A[i] > 0
# 3. -A[i+1]+A[i]-A[i-1] > 0

# for i in range(1, N-1):
#     p1, p2, p3 = A[i-1], A[i], A[i+1]
#     if p1+p2 > 0 or p2+p3 > 0 or -p1+p2-p3 > 0:
#         B[i] = abs(p2)
#     else:
#         B[i] = -abs(p2)

# B[0] = A[0]+A[1] if A[0]+A[1] > 0 else -A[0]-A[1]
# B[-1] = A[-1]+A[-2] if A[-1]+A[-2] > 0 else -A[-1]-A[-2]

# print(sum(B))

while True:
    ch = False
    for i in range(1, N-1):
        # A[i]+A[i+1]+A[i+2] < -A[i]+A[i+1]-A[i+2]:
        if A[i-1] + A[i] + A[i+1] < -A[i-1]+A[i]-A[i+1]:
            A[i-1] *= -1
            A[i+1] *= -1
            ch = True
        if A[i+1] + A[i] < 0:
            A[i+1] *= -1
            A[i] *= -1
            ch = True            
    if A[0] + A[1] < 0:
        A[0] *= -1
        A[1] *= -1
        ch = True
    if ch is False:
        break

print(sum(A))

