#-*- coding:utf-8 -*-

# def print_map(mp):
#     for r in mp:
#         print(['{:2d}'.format(n) for n in r])

H, W = map(int, input().split())
mp = [[] for h in range(H)]
mp_row = [[0 for w in range(W)] for h in range(H)]
mp_col = [[0 for w in range(W)] for h in range(H)]

# True = なし、 False = 障害物
for h in range(H):
    mp[h] = [True if c == "." else False for c in input()]

# 全探索はO(HWHW)となって爆発。
# 縦横でまずカウントするのがよさげ。
# このプログラムはO(HW)だがTLE: カウントしたものの書き込みでO(HHW+HWW)あたりになっている？
# これpythonのforの遅さでTLEしてないか…？
# →PyPyで出したらAC。まじかー。

## 横のパネル数
for h in range(H):
    st = 0
    cnt = 0
    for w in range(W):
        if mp[h][w] is False:
            for i in range(st, w):
                mp_row[h][i] = cnt
            cnt = 0
            st = w+1
        elif w == W-1:
            cnt += 1
            for i in range(st, W):
                mp_row[h][i] = cnt
        else:
            cnt += 1

## 縦のパネル数
for w in range(W):
    st = 0
    cnt = 0
    for h in range(H):
        if mp[h][w] is False:
            for i in range(st, h):
                mp_col[i][w] = cnt
            cnt = 0
            st = h+1
        elif h == H-1:
            cnt += 1
            for i in range(st, H):
                mp_col[i][w] = cnt
        else:
            cnt += 1

#print_map(mp_row)
#print()
#print_map(mp_col)

mx = 0
for h in range(H):
    for w in range(W):
        # 重なっている中央を引くのを忘れずに
        mx = max(mx, mp_row[h][w]+mp_col[h][w]-1)
print(mx)
