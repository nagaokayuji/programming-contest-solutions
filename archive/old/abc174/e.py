import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, K = readmi()
    A = readli()

    ok = 10**9
    ng = 0

    @njit(b1(i8, i8[:], i8), cache=True)
    def is_ok(N, A, l):
        cut_count = 0
        for i in range(N):
            x = A[i]
            cut_count += (x+l-1)//l - 1
            if cut_count > K:
                return False
        return True

    Aa = np.array(A)
    while ok-ng > 1:
        mid = (ok+ng)//2
        if is_ok(N, Aa, mid):
            ok = mid
        else:
            ng = mid
    print(ok)


if __name__ == '__main__':
    _solve()
