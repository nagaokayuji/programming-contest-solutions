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
    N = int(input())

    def sumlr(l, r):
        return (r*(r+1)//2 - l*(l-1)//2) % MOD
    ans = 0
    base = 1
    while base * 10 - 1 < N:
        ans += sumlr(0, base*9)
        ans %= MOD
        base *= 10
    ans += sumlr(0, N-base+1)
    ans %= MOD
    print(ans)


if __name__ == '__main__':
    MOD = 998244353
    _solve()
