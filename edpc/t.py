import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


@njit("i8(i8, b1[:])", cache=True)
def solve(N, largers):
    MOD = 1000000007
    dp = np.zeros((N, N+1), dtype=np.int64)
    dp[0][0] = 1

    for j in range(1, N+1):
        dp[0][j] = 1

    for i in range(1, N):
        larger = largers[i-1]
        cum = np.cumsum(dp[i-1]) % MOD
        for j in range(i+1):
            if larger:
                dp[i][j] += cum[j-1] if j > 0 else 0
            else:
                dp[i][j] += cum[i-1] - (cum[j-1] if j > 0 else 0)
        dp[i] %= MOD

    ans = 0
    for i in range(N):
        ans += dp[N-1][i]
        ans %= MOD
    return ans


def _solve():
    N = int(input())
    S = input()
    largers = np.array(list(map(lambda x: x == "<", list(S))))

    print(solve(N, largers))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
