# -*- coding: utf-8 -*-

S = input()

# 最終状態からの考察による解法
# 最終出力は1のみか0のみとなるはず
# つまり、0と1の個数をカウントし、その差分から答えが求められる

c0 = S.count("0")
c1 = S.count("1")

fin = abs(c0-c1)    # 最後に残る0か1の個数
res = len(S) - fin

print(res)
