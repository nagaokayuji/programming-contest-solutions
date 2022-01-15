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
    AB = [readti() for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for a, b in AB:
        a -= 1
        b -= 1
        g[a].append(b)
        g[b].append(a)

    pq = [0]
    anss = []
    visited = [False]*N
    visited[0] = True
    while pq:
        nx = heappop(pq)
        anss.append(nx+1)
        for cango in g[nx]:
            if not visited[cango]:
                heappush(pq, cango)
                visited[cango] = True

    print(*anss)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
