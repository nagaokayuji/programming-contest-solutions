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
    K = readi()
    now = 0
    # count = 0
    s = set()
    for count in range(1, K+1):
        now = (now*10 + 7) % K
        if now in s:
            print(-1)
            return
        s.add(now)
        # count += 1
        # print(now)
        if now % K == 0:
            print(count)
            return
    print(-1)
    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
