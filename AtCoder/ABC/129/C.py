#-*- coding:utf-8 -*-

N, M = map(int, input().split())
hsh = {}
for i in range(M):
    hsh[int(input())] = True

acc = [0 for _ in range(N+1)]
acc[0] = 1
if 1 in hsh:
    acc[1] = 0
else:
    acc[1] = 1

for i in range(2,N+1):
    if i-2 in hsh:  # 2段前が穴
        if i-1 in hsh: # 1段前が穴
            d = 0
        else:
            d = acc[i-1]
    else:  # 2段前が穴ではない
        if i-1 in hsh:  # 1段前が穴
            d = acc[i-2]
        else: 
            d = acc[i-1] + acc[i-2]
    acc[i] = d

# print(hsh)
# print(acc)
print(acc[-1]%1000000007)
