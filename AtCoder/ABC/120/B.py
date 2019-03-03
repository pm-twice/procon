# -*- coding: utf-8 -*-

A, B, K = map(int, input().split())

yakuA = [i for i in range(1, 101) if A % i == 0]
yakuB = [i for i in yakuA if B % i == 0]

print(yakuB[-K])
