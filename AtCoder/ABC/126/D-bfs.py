# -*- coding: utf-8 -*-

from collections import deque

N = int(input())
L = N-1
graph = [[] for _ in range(N)]
color = [-1 for _ in range(N)]
visit = [False for _ in range(N)]

for i in range(L):
    u,v,w = map(int, input().split())
    u-=1
    v-=1
    graph[u].append((v,w))
    graph[v].append((u,w))

que = deque([0])    # 添え字0からアクセス開始
color[0] = 0    # 添え字0をcolor0と置く
while len(que) > 0:
    idx = que.popleft()
    visit[idx] = True
    for i, w in graph[idx]:
        if visit[i] is True:
            continue
        if w%2 == 0:    # 同色で塗る
            color[i] = color[idx]
            que.append(i)
        else:   # 異色で塗る
            color[i] = 1 - color[idx]   # 1-cで異色になる
            que.append(i)

for c in color:
    print(c)
