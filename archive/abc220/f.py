import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    UV = [readti() for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for u, v in UV:
        u -= 1
        v -= 1
        g[u].append(v)
        g[v].append(u)

    stack = [(0, None, 0)]
    sum_from_0 = 0
    while stack:
        v, prev, dist = stack.pop()
        sum_from_0 += dist
        for nx in g[v]:
            if nx != prev:
                stack.append((nx, v, dist+1))

    anss = [None]*N
    anss[0] = sum_from_0

    sizes = [1]*N

    def dfs(v, prev):
        ret = 1
        for nx in g[v]:
            if nx == prev:
                continue
            ret += dfs(nx, v)

        sizes[v] = ret
        return ret

    dfs(0, None)

    def dfs2(v, prev):
        for nx in g[v]:
            if nx == prev:
                continue
            anss[nx] = anss[v] + N - 2 * sizes[nx]
            dfs2(nx, v)

    dfs2(0, None)
    print(*anss)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
