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


class FenwickTree:
    def __init__(self, n: int = 10**6):
        self._n = n
        self.data = [0] * n

    def add(self, p: int, x):
        p += 1
        while p <= self._n:
            self.data[p-1] += x
            p += p & -p

    def sum(self, left: int, right: int):
        return self._sum(right) - self._sum(left)

    def _sum(self, r: int):
        s = 0
        while r > 0:
            s += self.data[r-1]
            r -= r & -r
        return s


def _solve():
    N, = li()
    S = list(input())
    Q, = li()
    Queries = [tuple(input().split()) for _ in range(Q)]

    offset = ord('a')
    bts = [FenwickTree(N+1) for _ in range(26)]
    for i in range(N):
        bts[ord(S[i]) - offset].add(i+1, 1)

    for type, iq, cq in Queries:
        iq = int(iq)
        if type == '1':
            ind = iq-1
            bts[ord(S[ind])-offset].add(ind+1, -1)

            S[ind] = cq
            bts[ord(S[ind])-offset].add(ind+1, 1)
        else:
            ret = 0
            left = int(iq)
            right = int(cq)
            for i in range(26):
                ret += min(1, bts[i].sum(left, right+1))
            print(ret)


if __name__ == '__main__':
    _solve()
