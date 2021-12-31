from pprint import pprint
import sys
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N = readi()
    x0, y0 = readti()
    xn2, yn2 = readti()

    midx = (x0+xn2)/2
    midy = (y0+yn2)/2

    distance_2 = (x0-midx)**2 + (y0-midy)**2
    th = 2*pi/N
    x0m = x0-midx
    y0m = y0-midy

    xmoved = cos(th)*x0m-sin(th)*y0m
    ymoved = sin(th)*x0m + cos(th)*y0m
    print(xmoved+midx, ymoved+midy)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
