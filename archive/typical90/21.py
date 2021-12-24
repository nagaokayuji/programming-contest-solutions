from typing import List
from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


class SCC:
    def __init__(self, n: int):
        ''' n: num of nodes'''
        self.n = n
        self.g = [[] for _ in range(n)]
        self.rg = [[] for _ in range(n)]

    def from_graph(self, g: List[List[int]]):
        self.n = n = len(g)
        self.g = g.copy()
        self.rg = [[] for _ in range(n)]
        for v in range(n):
            for to in g[v]:
                self.rg[to].append(v)

    def add_edge(self, _from: int, to: int):
        self.g[_from].append(to)
        self.rg[to].append(_from)

    def scc(self):
        n = self.n
        group = [None] * n  # トポロジカル順序
        used = [False] * n
        order = []  # 帰りがけ順の並び

        def dfs(v: int):
            used[v] = True
            for nx in self.g[v]:
                if not used[nx]:
                    dfs(nx)
            order.append(v)

        def rdfs(v: int, k: int):
            used[v] = True
            group[v] = k
            for nx in self.rg[v]:
                if not used[nx]:
                    rdfs(nx, k)

        for v in range(n):
            if not used[v]:
                dfs(v)
        used = [False] * n
        k = 0
        for i in range(n-1, -1, -1):
            if not used[order[i]]:
                rdfs(order[i], k)
                k += 1
        return k, group


N, M = mi()
AB = [tuple(mi()) for _ in range(M)]

g = [[] for _ in range(N)]
for a, b in AB:
    g[a-1].append(b-1)

scc = SCC(N)
scc.from_graph(g)
k, group = scc.scc()
gc = [0]*N
for gr in group:
    gc[gr] += 1

ans = 0
for i in range(N):
    ans += gc[i]*(gc[i]-1)//2
print(ans)
