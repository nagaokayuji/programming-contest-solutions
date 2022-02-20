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
def yn(a): print("Takahashi" if a else "Aoki")


def sieve(n):
    is_prime = [True for _ in range(n+1)]
    is_prime[0] = False
    is_prime[1] = False

    for i in range(2, n+1):
        if is_prime[i]:
            j = i + i
            while j <= n:
                is_prime[j] = False
                j += i
    return is_prime


def _solve():
    ''''''
    is_prime = sieve(10**4)
    A, B, C, D = li()

    # 素数にできない やつ
    for takahashi in range(A, B+1):
        takahashi_win = True
        for aoki in range(C, D+1):
            if is_prime[takahashi + aoki]:
                takahashi_win = False
        if takahashi_win:
            yn(1)
            return
    yn(0)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
