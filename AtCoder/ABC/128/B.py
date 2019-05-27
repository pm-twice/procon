#-*- coding:utf-8 -*-

# Pythonのソートが安定であることを利用する
# まず、得点で降順でソートして、市名で昇順ソートする

N = int(input())
ls = [None]*N
for i in range(N):
    s, p = input().split()
    ls[i] = (i+1, s, int(p))
ls.sort(key=(lambda x: -x[2]))  # pで降順ソート
ls.sort(key=(lambda x: x[1]))   # sで昇順ソート (安定ソートなので、同じsについてpが降順となる)

for i, s, p in ls:
    print(i)


