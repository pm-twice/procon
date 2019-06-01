# -*- coding: utf-8 -*-

# ハッシュテーブルを使って毎回の探索を短縮。
# 計算量は時間がO(Q), 空間がO(N)となるはず。
# やはり実行時間超過。定数倍かどちらかをlogオーダーにする必要あり？
# pypy3でもダメだった。

N, Q = map(int, input().split())
s = input()
hsh = {}
glm = [1] * N

# ハッシュテーブルで文字ごとのindexを記憶: O(N)
for i in range(N):
    if s[i] not in hsh:
        hsh[s[i]] = [i]
    else:
        hsh[s[i]].append(i)
#print(hsh)

# 呪文ごとに操作: O(Q)
# 厳密にはハッシュ探索時間と、リスト平均長もかかる
# 最悪の場合(全てマスの文字が同じ)は、O(QN)になってしまう。 
for i in range(Q):
    t, d = input().split()
    #print(glm)
    if t in hsh:
        if d == "L":
            for i in hsh[t]:
                if i > 0:
                    glm[i-1] += glm[i]
                glm[i] = 0
        else:   # d == "R"
            for i in hsh[t]:
                if i < N-1:
                    glm[i+1] += glm[i]
                glm[i] = 0

print(sum(glm))
