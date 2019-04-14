# -*- coding: utf-8 -*-

N, K = map(int, input().split())
S = input()

# N, Kがそれぞれ10^5以下なので、O(NK)は不可能
# 1の連続Bit数を最大化する
# K回の操作で、ある区間におけるK個の00を11に変えることに相当。
# (001100を110011にして、111111にするのも結局2回要している)

# 00,11のグループで再カウント、[(start, end, True/False)]: Trueなら1
start = 0
l = []
for i in range(1,N):
    if S[i-1] == S[i]:
        continue
    else:
        v = True if S[i-1] == "1" else False
        l.append((start, i, v))
        start = i
v = True if S[-1] == "1" else False
l.append((start, N, v))

mx = 0
L = len(l)
for i, val in enumerate(l):
    start, end, v = val
    if v is True:   # "1"
        # 1 0 1 0 1として、1+2Kのグループを1にできる
        idx = min(i+2*K, L-1)
    else:   # "0"
        # 0 1 0 1として、2Kのグループを1にできる
        idx = min(i+2*K-1, L-1)
    ln = l[idx][1] - start
    if ln > mx:
        mx = ln

print(mx)
