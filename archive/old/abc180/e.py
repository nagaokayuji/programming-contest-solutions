import sys
from pprint import pprint
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = 10**16
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@njit(i8(i8[:], i8[:], i8[:], i8), cache=True)
def solve(X, Y, Z, N):
    dp = np.full((1 << N, N), INF, dtype=np.int64)
    dp[0, 0] = 0
    dp[1, 0] = 0
    for bits in range(1 << N):
        for ot in range(N):
            for to in range(N):
                if ot == to:
                    continue
                a, b, c = X[ot], Y[ot], Z[ot]
                p, q, r = X[to], Y[to], Z[to]
                dp[bits | (1 << to)][to] = min(
                    dp[bits | (1 << to)][to], dp[bits][ot] + abs(p-a)+abs(q-b) + max(0, r-c))

    return dp[-1][0]


def _solve():
    N = readi()
    XYZ = np.array([readti() for _ in range(N)])
    dp = [[INF]*N for _ in range(1 << N)]
    dp[0][0] = 0
    dp[1][0] = 0
    print(solve(XYZ[:, 0], XYZ[:, 1], XYZ[:, 2], N))


if __name__ == '__main__':
    _solve()
