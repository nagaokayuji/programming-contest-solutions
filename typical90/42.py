import sys
from numba import njit, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


MOD = 10**9 + 7

K = int(input())


@njit("void(i8)")
def solve(K):
    dp = np.zeros((K+1, 9), dtype=np.int64)
    dp[0][0] = 1
    for ketawa in range(K):
        for mod9 in range(9):
            dp[ketawa][mod9] %= MOD
            for ad in range(1, 10):
                if ketawa+ad > K:
                    break
                dp[ketawa+ad][(mod9+ad) % 9] += dp[ketawa][mod9]
                dp[ketawa+ad][(mod9+ad) % 9] %= MOD
    dp %= MOD
    print(dp[K][0])


solve(K)
