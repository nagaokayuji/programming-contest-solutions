import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, K = readmi()
    A = readli()
    asum = np.cumsum([0]+A)
    ans = 0
    for l in range(N-K+1):
        ans += asum[l+K] - asum[l]
    print(ans)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
