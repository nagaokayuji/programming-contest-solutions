import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial, pi, cos, sin, hypot, tan
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


def _solve():
    A, B, X = li()

    def calc(rad):
        if tan(rad)*A <= B:
            return A*A*B - A*A*A*tan(rad)/2
        return B*B/tan(rad)*A/2

    ok = 0
    ng = pi/2
    for _ in range(70):
        mid = (ok+ng)/2
        if calc(mid) >= X:
            ok = mid
        else:
            ng = mid
    print(ok*180/pi)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
