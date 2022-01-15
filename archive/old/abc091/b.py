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
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())
def li(): return list(mi())
def li1(): return list(mi1())
def ti(): return tuple(mi())
def ti1(): return tuple(mi1())


def _solve():
    N = int(input())
    s = [input() for _ in range(N)]
    M = int(input())
    t = [input() for _ in range(M)]
    d = defaultdict(int)
    for x in s:
        d[x] += 1
    for x in t:
        d[x] -= 1

    ans = max(0, max(d.values()))
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
