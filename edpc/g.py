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
    xy = [tuple(mi1()) for _ in range(M)]
    g = [[] for _ in range(N)]
    for x, y in xy:
        g[x].append(y)

    dp = [INF]*N

    def dfs(v):
        if not g[v]:
            dp[v] = 0
            return 0
        if dp[v] != INF:
            return dp[v]

        ret = 0
        for nx in g[v]:
            ret = max(ret, dfs(nx)+1)
        dp[v] = ret
        return ret

    for x in range(N):
        dfs(x)
    print(max(dp))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
