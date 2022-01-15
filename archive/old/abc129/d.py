import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    H, W = mi()
    S = [input() for _ in range(H)]

    row = [[0]*W for _ in range(H)]
    col = [[0]*W for _ in range(H)]
    for i in range(H):
        for j in range(W):
            if j == 0:
                if S[i][j] == '.':
                    row[i][j] += 1
            else:
                if S[i][j] == '.':
                    row[i][j] = row[i][j-1]+1
        for j in range(W-2, -1, -1):
            if row[i][j] > 0:
                row[i][j] = max(row[i][j], row[i][j+1])

    for j in range(W):
        for i in range(H):
            if i == 0:
                if S[i][j] == '.':
                    col[i][j] += 1
            else:
                if S[i][j] == '.':
                    col[i][j] = col[i-1][j]+1
        for i in range(H-2, -1, -1):
            if col[i][j] > 0:
                col[i][j] = max(col[i][j], col[i+1][j])

    ans = 0
    for i in range(H):
        for j in range(W):
            ans = max(ans, row[i][j]+col[i][j]-1)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
