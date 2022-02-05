import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, K = mi()
    A = list(mi())
    dp = [0]*(K+1)
    dp[0] = 1
    dpsum = [1]*(K+2)
    dpsum[0] = 0

    for a in A:
        ndp = [0]*(K+1)
        ndpsum = [0]*(K+2)
        for k in range(K+1):
            ndp[k] = dpsum[k+1] - dpsum[max(0, k-a)]
            ndp[k] %= MOD
            ndpsum[k+1] += ndpsum[k] + ndp[k] % MOD
            ndpsum[k+1] %= MOD
        dp = ndp
        dpsum = ndpsum
    # print(*dp)
    print(dp[K] % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
