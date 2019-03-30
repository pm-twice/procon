# -*- coding: utf-8 -*-

# 別の考察でトライ。
# あるマスの値が右に落ちるなら右のマスすべても落ちる。左に落ちる時も同様
# この仕様を用いてどうにか短縮できないか…？
# とは言えこれが分かるのもQがすべて完了した後。
# O(Q)は避けられず、定数倍をいかに短縮するかという話なのか…？
# O(N)のところをどうやって短くするかという話？
# 英大文字毎に移動量を算出しても、逐次性消えるので左右文字からの移動が判別できず正確にならない。
# マスごとに左右落ちるか二分探索出来ればいいのか？
#
# 1. 中央マスを選択
# 2. 中央マスのゴーレムの移動にのみ着目
# 3. 移動終了後、その最右と最左のマスに着目して同様の手順
# これも逐次性でダメか…？

from collections import deque

N, Q = map(int, input().split())
s = input()
glm = [1] * N
que = deque([])

T, D = [0] * Q, [0] * Q
for i in range(Q):
    T[i], D[i] = input().split()

que.append([0, N//2, N])
while len(que) > 0:
    print(glm)
    mn, id, mx = que.pop()
    i = id
    for j in range(Q):
        if i < mn or i >= mx-1:
            break
        if s[i] == T[j]:
            if D[j] == "R":
                if i < N-2:
                    glm[i+1] += glm[i]
                glm[i] = 0
                i+=1
            else:
                if i > 0:
                    glm[i-1] += glm[i]
                glm[i] = 0
                i-=1
    if i >= 0 and id-mn > 1:
        que.append([mn, (id+mn)//2, id])
    if i <= N-1 and mx-id > 1:
        que.append([id, (id+mx)//2, mx])

print(sum(glm))
