
import numpy as np
from numba import njit
N, M = map(int, input().split())
# print(N, M)
MOD = 998244353
N = N*2

AB = np.array([tuple(map(int, input().split())) for _ in range(M)])
print(AB)
# print(dp)
dp = [[None] * (N+1) for _ in range(N)]
g = [[] for _ in range(N)]

for a, b in AB:
    if a+1 == b:
        dp[a-1][b-1] = 1
    else:
        g[a-1].append(b-1)
        g[b-1].append(a-1)


def rec(l, r):
    if r-l <= 1:
        return 0
    if dp[l][r] is not None:
        return dp[l][r]
    if r-l <= 2:
        return dp[l][r]
    ret = 0
    for i in range(l+1, r):
        ret += rec(l+1, i) * rec(i

    return dp[l][r]


print(dp[0][N-1])
