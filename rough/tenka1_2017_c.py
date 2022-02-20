import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from functools import lru_cache
from math import sqrt, gcd, factorial, pi, cos, sin, hypot
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def わーい():
    '''
    4/N = 1/h + 1/n + 1/w
    4*hnw = N(nw + hw + hn)
    (4hn-Nn-Nh)w = Nhn
    '''
    N, = li()
    for h in range(1, 3501):
        for n in range(1, 3501):
            l = 4*h*n - N*n - N*h
            r = N*h*n
            if (not l) or (r % l != 0):
                continue
            w = r//l
            if w > 0:
                print(h, n, w)
                return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    わーい()
