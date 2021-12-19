from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
#from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N, M = mi()
ABC = [tuple(mi()) for _ in range(M)]

g = [[] for _ in range(N)]
for a, b, c in ABC:
    g[a-1].append((b-1, c))
    g[b-1].append((a-1, c))

INF = 10**18


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


from1 = dijkstra(g, 0)
fromn = dijkstra(g, N-1)

for i in range(N):
    t = from1[i] + fromn[i]
    print(t)
