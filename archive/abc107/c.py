import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, K = readmi()
    X = readli()
    minus = []
    plus = []
    for x in X:
        if x < 0:
            minus.append(-x)
        if x > 0:
            plus.append(x)
        if x == 0:
            K -= 1
    if K == 0:
        print(0)
        return
    minus.sort()
    plus.sort()

    minuss = [0]+minus
    pluss = [0]+plus
    ans = INF

    for plusc in range(K+1):
        minusc = K-plusc
        if plusc > len(plus) or minusc > len(minus):
            continue
        ans = min(ans, minuss[minusc]*2+pluss[plusc])
        ans = min(ans, minuss[minusc]+pluss[plusc]*2)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
