#-*- coding:utf-8 -*-

import sys
sys.setrecursionlimit(10000)

def dfs(graph, used, color, v, c):
    color[v] = c    # 指定された色で塗る
    used[v] = True
    for e in graph[v]:
        if used[e[0]]:   # 訪問済みのノードは無視
            continue
        if e[1] % 2 == 1:   # 奇数。親と異色で塗る
            dfs(graph, used, color, e[0], 1-c)   # c=1or0なのでこれで逆に出来る
        else:   # 偶数。親と同色で塗る
            dfs(graph, used, color, e[0], c)


N = int(input())
graph = [[]]*N
used = [False]*N
color = [-1]*N

for i in range(N-1):
    u, v, w = map(int, input().split())
    u-=1
    v-=1
    graph[u].append((v, w))
    graph[v].append((u, w))


dfs(graph, used, color, 0, 0)    # 0を根と仮定する
for c in color:
    print(c)


