import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def わーい():
    ''''''
    N, M = li()
    ABT = [li() for _ in range(M)]
    dp = [[10**14]*N for _ in range(N)]
    for a, b, t in ABT:
        a -= 1
        b -= 1
        dp[a][b] = min(dp[a][b], t)
        dp[b][a] = min(dp[b][a], t)
    for i in range(N):
        dp[i][i] = 0

    dp = np.array(dp, dtype=np.int64)

    @njit(i8[:, :](i8[:, :]), cache=True)
    def calc(dp):
        for k in range(N):
            for i in range(N):
                for j in range(N):
                    dp[i][j] = min(dp[i][j], dp[i][k]+dp[k][j])
        return dp
    dp = calc(dp)

    ans = INF
    for i in range(N):
        tmp = 0
        for j in range(N):
            if i == j:
                continue
            tmp = max(tmp, dp[i][j])
        ans = min(ans, tmp)

    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    わーい()
