import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def わーい():
    N, = li()
    S = [list(map(int, list(input()))) for _ in range(N)]

    for k in range(N):
        for i in range(N):
            for j in range(N):
                S[i][j] |= S[i][k] and S[k][j]

    ans = 0

    for i in range(N):
        count = 1
        for j in range(N):
            if i != j and S[j][i]:
                count += 1
        ans += 1/count
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    わーい()
