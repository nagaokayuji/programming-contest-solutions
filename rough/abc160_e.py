import sys
from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
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


def _solve():
    X, Y, A, B, C = li()
    P = sorted(li(), reverse=True)[:X]
    Q = sorted(li(), reverse=True)[:Y]
    R = sorted(li(), reverse=True)
    pq = []
    for x in P:
        heappush(pq, x)
    for y in Q:
        heappush(pq, y)

    for r in R:
        if pq[0] < r:
            heappop(pq)
            heappush(pq, r)

    ans = sum(pq)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
