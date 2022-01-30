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
    N, M = mi()
    H = list(mi())

    def f(x, y):
        b = H[x]-H[y]
        if b > 0:
            return b
        else:
            return b*2

    g = [[] for _ in range(N)]
    UV = [tuple(mi1()) for _ in range(M)]
    for u, v in UV:
        g[u].append((v, -f(u, v)))
        g[v].append((u, -f(v, u)))

    def dijkstra(g, start):
        dists = [10**18] * N
        dists[start] = 0
        que = [(0, start)]
        while que:
            p = heappop(que)
            c = p[0]
            v = p[1]
            if dists[v] < c:
                continue
            for to, cost in g[v]:
                if dists[to] > dists[v] + cost:
                    dists[to] = dists[v] + cost
                    heappush(que, (dists[to], to))
        return dists

    ret = dijkstra(g, 0)
    print(-min(ret))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
