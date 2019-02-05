# -*- coding: utf-8 -*-
# 解答例を基にmap関数を利用して書き換え
a = int(input())
b, c = map(int, input().split())
s = input()
print("{0} {1}".format(a+b+c, s))
