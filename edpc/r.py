import sys
# from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def pow(A, k):
    n = len(A)
    base = A
    ret = np.eye(n, dtype=object)
    while k:
        if k & 1 == 1:
            ret = ret @ base
            ret %= MOD
        base = base @ base
        base %= MOD
        k >>= 1
    return ret


def _solve():
    N, K = mi()
    A = np.matrix([list(mi()) for _ in range(N)], dtype=object)
    res = pow(A, K)
    print(np.sum(res) % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
