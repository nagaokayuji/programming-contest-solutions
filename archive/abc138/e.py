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
    s = input()
    n = len(s)
    t = input()
    m = len(t)

    s = s+s
    poss = [[] for _ in range(26)]

    for i, c in enumerate(s):
        cind = ord(c) - ord('a')
        poss[cind].append(i)

    prev = -1
    count = 0
    for c in t:
        cind = ord(c) - ord('a')
        if not poss[cind]:
            print(-1)
            return
        i = bisect_left(poss[cind], prev+1)
        prev = poss[cind][i]
        if prev >= n:
            prev %= n
            count += 1
    print(prev+1 + n*count)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
