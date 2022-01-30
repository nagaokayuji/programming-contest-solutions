import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, L, W = mi()
    A = list(mi())
    difs = []

    if A[0] != 0:
        difs.append(A[0])
    for i in range(N-1):
        difs.append(A[i+1]-A[i] - W)
    if A[-1] != L:
        difs.append(L - A[-1] - W)

    ans = 0
    for x in difs:
        if x > 0:
            ans += (x+W-1)//W

    print(ans)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
