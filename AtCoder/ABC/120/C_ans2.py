# -*- coding: utf-8 -*-
from collections import deque

S = input()

# スタックを用いた解法
# ()の整合性判定にも使える

que = deque()
res = 0
for d in S:
    if len(que) == 0:
        que.append(d)
    elif que[-1] != d:  # 01 or 10
        res += 2
        que.pop()   # ペアを作成
    else:
        que.append(d)

print(res)
