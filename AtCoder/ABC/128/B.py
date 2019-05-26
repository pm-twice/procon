#-*- coding:utf-8 -*-

N = int(input())
ls = {}
for i in range(N):
    s, p = input().split()
    if s not in ls:
        ls[s] = [(i+1, int(p))]
    else:
        ls[s].append((i+1, int(p)))

r = [(k,v) for k, v in ls.items()]
r.sort(key=(lambda x: x[0]))

for l in r:
    vals = l[1]
    vals.sort(key=(lambda x: x[1]), reverse=True)
    for v in vals:
        print(v[0])


