# -*- coding:utf-8 -*-
# 白昼夢 / Daydream
# 再帰関数版

divide = ["dream", "dreamer", "erase", "eraser"]
S = input()


def rec(s):
    if s == "":     # len(s) >= 1
        return True
    else:
        for d in divide:
            if s.startswith(d) and rec(s[len(d):]):
                return True
        return False

if rec(S) is True:
    print("YES")
else:
    print("NO")

raise RuntimeError("メモリオーバーで動作しない？")
