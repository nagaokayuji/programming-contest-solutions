import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


@njit(void(i8, i8[:]))
def solve(N, cumsum):
    dp = np.zeros((N+1, N+1), dtype=np.int64)
    for length in range(1, N+1):
        for left in range(N-length+1):
            right = left+length  # [left,right)
            t = INF
            if length == 1:
                t = 0
            for mid in range(left+1, right):
                t = min(t, dp[left][mid]+dp[mid][right])
            dp[left][right] = t + cumsum[right]-cumsum[left]
    print(dp[0][N] - cumsum[N])


def _solve():
    N = int(input())
    A = list(mi())
    cumsum = np.cumsum([0]+A)
    solve(N, cumsum)


if __name__ == '__main__':
    _solve()
