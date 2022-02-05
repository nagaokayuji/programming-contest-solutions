import sys
# from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


# @njit("i8(i8, b1[:])", cache=True)
def solve(N, largers):
    MOD = 1000000007
    dp = np.zeros((N, N+1), dtype=np.int64)
    dp[0][0] = 1

    for i in range(1, N):
        larger = largers[i-1]
        cum = np.cumsum(dp[i-1]) % MOD
        if larger:
            dp[i, 1:i+1] = dp[i, 1:i+1] + cum[:i]
        else:
            dp[i, : i+1] = dp[i, : i+1] + cum[i-1]
            dp[i, 1:i+1] -= cum[:i]
        dp[i] %= MOD

    return np.sum(dp[N-1]) % MOD


def _solve():
    N = int(input())
    S = input()
    largers = np.array(list(map(lambda x: x == "<", list(S))))

    print(solve(N, largers))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
