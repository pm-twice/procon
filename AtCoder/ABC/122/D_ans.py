# -*- coding: utf-8 -*-

N = int(input())

if N == 3:
    print(61)
    exit()

MOD = 10**9 + 7
dp = [[0]*64 for i in range(N-2)]
A,C,G,T = 0,1,2,3

# N=3の場合
for x in range(4):
    for y in range(4):
        for z in range(4):
            if x==A and y==G and z==C:
                continue
            elif x==G and y==A and z==C:
                continue
            elif x==A and y==C and z==G:
                continue
            else:
                dp[0][x*16+y*4+z] = 1

# N > 3の場合
for i in range(N-3):
    # 4文字に着目=256
    for j in range(64):
        a, b = divmod(j, 16)
        b, c = divmod(b, 4)
        for d in range(4):
            if b==A and c==G and d==C: continue
            elif b==A and c==C and d==G: continue
            elif b==G and c==A and d==C: continue
            elif a==A and b==G and d==C: continue
            elif a==A and c==G and d==C: continue
            dp[i+1][b*16+c*4+d] += dp[i][j]
            dp[i+1][b*16+c*4+d] %= MOD

print(sum(dp[-1])%MOD)
