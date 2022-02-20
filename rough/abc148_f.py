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


def _solve():
    N, U, V = li()
    U -= 1
    V -= 1
    AB = [tuple(li1()) for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for a, b in AB:
        g[a].append(b)
        g[b].append(a)

    def dist(st):
        dists = [INF]*N
        dists[st] = 0

        def dfs(v=st, prv=None):
            for to in g[v]:
                if to == prv:
                    continue
                dists[to] = dists[v]+1
                dfs(to, v)

        dfs()
        return dists

    from_u = dist(U)
    from_v = dist(V)

    ans = 0
    for i in range(N):
        if from_u[i] >= from_v[i]:
            continue
        t = from_v[i]-1
        ans = max(ans, t)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
