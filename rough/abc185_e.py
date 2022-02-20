import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    N, M = li()
    A = li()
    B = li()

    dp = [[INF]*(M+1) for _ in range(N+1)]
    dp[0][0] = 0
    for i in range(N+1):
        for j in range(M+1):
            if j < M:
                dp[i][j+1] = min(dp[i][j+1], dp[i][j]+1)
            if i < N:
                dp[i+1][j] = min(dp[i+1][j], dp[i][j]+1)
            if i < N and j < M:
                if A[i] != B[j]:
                    dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j]+1)
                else:
                    dp[i+1][j+1] = min(dp[i+1][j+1], dp[i][j])

    print(dp[N][M])


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
