import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from functools import lru_cache
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


leafc = 0


def _solve():
    ''''''
    N, = li()
    UV = [li1() for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for u, v in UV:
        g[u].append(v)
        g[v].append(u)

    sizes = [1]*N
    sizes[0] = N
    anss = [(INF, 0)] * N

    def dfs(v, prev=-1):
        global leafc
        sz = 1
        l = INF
        r = 0
        for to in g[v]:
            if to == prev:
                continue
            dfs(to, v)
            sz += sizes[to]
            ll, rr = anss[to]
            l = min(l, ll)
            r = max(r, rr)

        if sz == 1:
            leafc += 1
            anss[v] = (leafc, leafc)
        else:
            anss[v] = (l, r)

        sizes[v] = sz
    dfs(0)
    # print(sizes)
    for l, r in anss:
        print(l, r)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
