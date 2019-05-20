# -*- coding: utf-8 -*-

import sys
sys.setrecursionlimit(100000)

def dfs(graph, color, visit, idx):
    visit[idx] = True
    if len(graph[idx]) == 0:
        return
    for i, w in graph[idx]:
        if visit[i] is True:
            continue
        if w%2 == 0:    # 同色で塗る
            color[i] = color[idx]
        else:   # 異色で塗る
            color[i] = 1 - color[idx]   # 1-cで異色になる
        dfs(graph, color, visit, i)


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

color[0] = 0    # 添え字0をcolor0と置く
dfs(graph, color, visit, 0)

for c in color:
    print(c)
