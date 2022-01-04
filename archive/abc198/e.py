import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readmi1(): return map(lambda x: int(x)-1, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readti1(): return tuple(readmi1())
def readi(): return int(input())


def _solve():
    N = readi()
    C = readli()
    AB = [readti1() for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for a, b in AB:
        g[a].append(b)
        g[b].append(a)

    anss = []

    usedc = defaultdict(int)

    def dfs(v, prev):
        usedc[C[v]] += 1
        if usedc[C[v]] == 1:
            anss.append(v)

        for nx in g[v]:
            if nx == prev:
                continue
            dfs(nx, v)
        usedc[C[v]] -= 1
    dfs(0, None)
    anss.sort()
    for x in anss:
        print(x+1)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
