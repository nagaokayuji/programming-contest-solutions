import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))


@njit(void(i8, i8, i8), cache=True)
def solve(N, X, Y):
    ret = np.zeros(N, np.int64)
    for i in range(N):
        for j in range(i+1, N):
            d = min(abs(j-i), abs(X-i)+abs(Y-j)+1, abs(Y-i)+abs(X-j)+1)
            ret[d] += 1

    for i in range(1, N):
        print(ret[i])


def _solve():
    N, X, Y = li()
    if X > Y:
        X, Y = Y, X
    solve(N, X-1, Y-1)


if __name__ == '__main__':
    _solve()
