import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N = int(input())
    A = np.array([list(mi()) for _ in range(N)], dtype=np.int64)

    @njit(i8(), cache=True)
    def solve():
        grouping_points = np.zeros(1 << N, dtype=np.int64)
        for bits in range(1 << N):
            points = 0
            for i in range(N):
                if bits >> i & 1 == 0:
                    continue
                for j in range(i+1, N):
                    if bits >> j & 1 == 0:
                        continue
                    points += A[i][j]
            grouping_points[bits] = points
        dp = np.zeros((1 << N), dtype=np.int64)
        for s in range(1 << N):
            k = s
            while k:
                dp[s] = max(dp[s], dp[s-k] + grouping_points[k])
                k = (k-1) & s
        return dp[(1 << N)-1]

    print(solve())


if __name__ == '__main__':
    _solve()
