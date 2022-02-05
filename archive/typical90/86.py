import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@njit(void(i8, i8[:], i8[:], i8[:], i8[:]), cache=True)
def calc(N, X, Y, Z, W):
    MOD = 1000000007
    Q = len(X)
    ret = 1
    for bit in range(60):
        cnt = 0
        for bits_n in range(1 << N):
            ok = True
            for qi in range(Q):
                x, y, z, w = X[qi], Y[qi], Z[qi], W[qi]
                target = w >> bit & 1
                ng = ((bits_n >> x & 1) or (bits_n >> y & 1) or (
                    bits_n >> z & 1)) != target
                if ng:
                    ok = False
                    break
            if ok:
                cnt += 1
        ret = ret*cnt % MOD
    print(ret)


def solve():
    N, Q = readti()
    XYZW = np.array([readli() for _ in range(Q)], dtype=np.int64)
    for i in range(Q):
        XYZW[i][0] -= 1
        XYZW[i][1] -= 1
        XYZW[i][2] -= 1
    X = XYZW[:, 0]
    Y = XYZW[:, 1]
    Z = XYZW[:, 2]
    W = XYZW[:, 3]
    calc(N, X, Y, Z, W)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    solve()
