from pprint import pprint
import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K = mi()
XY = [ti() for _ in range(N)]

d = np.zeros((N, N), dtype=np.int64)
for i in range(N):
    for j in range(N):
        d[i][j] = (XY[i][0]-XY[j][0])**2 + (XY[i][1]-XY[j][1])**2

M = 1 << N


@njit(i8(i8, i8, i8[:, :]), cache=True)
def solve(N, M, d):
    dist = np.zeros(M, dtype=np.int64)
    for bits in range(M):
        for i in range(N):
            for j in range(i):
                if ((bits >> i) & 1) and ((bits >> j) & 1):
                    dist[bits] = max(dist[bits], d[i][j])
    dp = np.full((M, K+1), 1000000000000000006, dtype=np.int64)
    dp[0][0] = 0
    for i in range(1, K+1):
        for j in range(M):
            k = j
            while k > 0:
                dp[j][i] = min(dp[j][i], max(dp[j-k][i-1], dist[k]))
                k = (k-1) & j
    return dp[M-1][K]


print(solve(N, M, d))
