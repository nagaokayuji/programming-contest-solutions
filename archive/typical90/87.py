from pprint import pprint
import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@njit(i8(i8[:, :], i8, i8), cache=True)
def get_by_x(A, x, P):
    n = len(A)
    dists = np.copy(A)
    for i in range(n):
        for j in range(n):
            if dists[i][j] == -1:
                dists[i][j] = x
    for k in range(n):
        for i in range(n):
            for j in range(n):
                dists[i][j] = min(dists[i][j], dists[i][k]+dists[k][j])
    ret = 0
    for i in range(n):
        for j in range(i+1, n):
            if dists[i][j] <= P:
                ret += 1
    return ret


@njit(i8(i8[:, :], i8, i8), cache=True)
def bs(A, K, P):
    ok = 0
    ng = 10**15
    while ng-ok > 1:
        mid = (ok+ng)//2
        if get_by_x(np.copy(A), mid, P) > K:
            ok = mid
        else:
            ng = mid
    return ok


def solve():
    N, P, K = readti()
    A = np.array([readli() for _ in range(N)])
    l = bs(np.copy(A), K, P)
    r = bs(np.copy(A), K-1, P)
    ans = r-l
    if ans >= 10**12:
        print("Infinity")
    else:
        print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
