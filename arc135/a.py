from functools import lru_cache
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
    X = int(input())

    @lru_cache(maxsize=None)
    def rec(x):
        if x <= 4:
            return x
        ret = rec((x+1)//2) * rec(x//2) % MOD
        return ret

    print(rec(X) % MOD)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 998244353
    _solve()
