import sys
import numpy as np
from numba import njit, void, b1, i1, i4, i8, f8
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())


MOD = 10**9+7
N, L = mi()


@njit(i8(i8, i8))
def solve(N, L):
    dp = np.zeros(N+1, dtype=np.int64)
    dp[0] = 1
    for i in range(N):
        dp[i] %= MOD
        dp[i+1] += dp[i]
        if i+L <= N:
            dp[i+L] += dp[i]
    return dp[N] % MOD


print(solve(N, L))
