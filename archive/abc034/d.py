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
def inputmi(): return map(int, input().split())
def inputmi1(): return map(lambda x: int(x)-1, input().split())
def inputli(): return list(inputmi())
def inputli1(): return list(inputmi1())
def inputti(): return tuple(inputmi())
def inputti1(): return tuple(inputmi1())
def inputi(): return int(input())


def _solve():
    N, K = inputti()
    WP = [inputti() for _ in range(N)]

    def isOK(r):
        a = []
        for w, p in WP:
            a.append(w*p/100 - w*r/100)
        a.sort(reverse=True)
        return sum(a[:K]) >= 0

    ok = 0
    ng = 100

    for _ in range(100):
        r = (ok+ng)/2
        if isOK(r):
            ok = r
        else:
            ng = r
    print(r)


if __name__ == '__main__':
    _solve()
