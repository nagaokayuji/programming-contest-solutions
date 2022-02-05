from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


sys.setrecursionlimit(10**6)

N = int(input())
AB = [ti() for _ in range(N-1)]
g = [[] for _ in range(N)]
for a, b in AB:
    a -= 1
    b -= 1
    g[a].append(b)
    g[b].append(a)


root = 0
sizes = [0]*N


def dfs(v, prev):
    sizes[v] += 1
    for nx in g[v]:
        if nx == prev:
            continue
        sizes[v] += dfs(nx, v)

    return sizes[v]


dfs(0, -1)

# pprint(sizes)

ans = 0
for i in range(N):
    ans += sizes[i]*(N-sizes[i])
print(ans)
