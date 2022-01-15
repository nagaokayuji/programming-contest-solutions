from pprint import pprint
import sys
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@njit(i8(i8[:], i8[:], i8, i8), cache=True)
def solve(As, Bs, K, N):
    ans = 0
    for a_count in range(N+1):
        if As[a_count] > K:
            break
        left = K - As[a_count]
        bc = searchsorted(Bs, left, side='right')-1
        ans = max(ans, a_count+bc)
    return ans


def _solve():
    N, M, K = readmi()
    A = readli()
    B = readli()
    As = np.cumsum([0]+A)
    Bs = np.cumsum([0]+B)
    print(solve(As, Bs, K, N))

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
